use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::stream::StreamExt;
use url::Url;
use futures_util::SinkExt;
use serde_json::{Value, json, Number};
use flate2::read::GzDecoder;
use std::io::Read;

use super::messages::{TickerJson, TradeJson, TradeSingletonJson, EmptyJson, Json};
use super::payload::Payload;

pub struct HuobiClient {
    payload: Payload,
    url: String,
    ping_delay: u8,
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn decompress_message(msg: Message) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    match msg {
        Message::Binary(data) => {
            let mut d = GzDecoder::new(&data[..]);
            let mut decompressed_data = Vec::new();
            d.read_to_end(&mut decompressed_data)?;
            Ok(decompressed_data)
        },
        _ => Err("Received non-binary message".into()),
    }
}

fn get_message(msg: Message) -> Result<String, Box<dyn std::error::Error>> {

    match decompress_message(msg) {
        Ok(data) => {
            match String::from_utf8(data) {
                Ok(v) => {return Ok(v);},
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };
        },
        Err(e) => panic!("Failed to decompress data: {}", e),
    };
}

impl HuobiClient {

    pub fn new() -> HuobiClient {
        HuobiClient {
            payload: Payload::new(),
            url: String::from("wss://api.huobi.pro/ws"),
            ping_delay: 60
        }
    }

    pub async fn run(&self) {

        let (ws_stream, response) =
        connect_async(Url::parse(&self.url).unwrap()).await.expect("Can't connect");

        println!("Connected to the server");
        println!("Response HTTP code: {}", response.status());
        println!("Response contains the following headers:");

        for (ref header, val /* value */) in response.headers() {
            println!("* {}: {:?}", header, val);
        }

        let (mut write, mut read) = ws_stream.split();
        let payload = self.payload.to_string();
        let mut channel: String = String::new();

        write.send(Message::Text(payload)).await.unwrap();
        
        'main_loop: 
        while let Some(Ok(raw_msg)) = read.next().await {

            let raw_msg = get_message(raw_msg);
            let msg = serde_json::from_str::<serde_json::Value>(&raw_msg.unwrap()).unwrap();

            if let Some(ping) = &msg.get("ping") {
                write.send(Message::Text(format!("{{\"pong\": \"{}\"}}", ping))).await.unwrap();
                continue 'main_loop;
            }

            channel.clear();

            if let Some(tmp) = HuobiClient::parse_channel(&msg) {
                channel = tmp;
            } else {
                continue 'main_loop;
            }          

            let tick_value: serde_json::Value;

            match &msg.get("tick") {
                Some(serde_json::Value::Object(obj)) => {
                    let mut m = obj.clone();

                    m.insert(String::from("ts"), json!(&msg.get("ts")));
                    tick_value = json!(m);
                },
                _ => {
                    continue 'main_loop;
                }
            };


            let data_text = serde_json::to_string::<serde_json::Value>(&tick_value).unwrap();

            let data: Box<dyn Json> = 
            
                if channel.contains("ticker") {

                    match serde_json::from_str::<TickerJson>(&data_text) {
                        Result::Ok(val) => Box::new(val),
                        Result::Err(err) => {println!("Error: {}", err); continue 'main_loop;}
                    }
                }

                else if channel.contains("trade.detail") {

                    let mut singleton_vec: Vec<TradeSingletonJson> = vec![];

                    'trade_loop:
                    for parsed_trade in (&tick_value.get(&"data").unwrap().as_array().unwrap()).iter(){

                        let string_trade = serde_json::to_string::<serde_json::Value>(parsed_trade).unwrap();

                        match serde_json::from_str::<TradeSingletonJson>(&string_trade) {
                            Result::Ok(value) => {singleton_vec.push(value)},
                            Result::Err(err) => {println!("Error: {}", err); continue 'trade_loop;}
                        }

                    };

                    Box::new(TradeJson::new(singleton_vec))
                }

                else {
                    Box::new(EmptyJson{})
                };

            let data_str = serde_json::to_string_pretty(&data).unwrap();
            println!("{}: {}", &channel, data_str);

        }
    }

    fn parse_channel(msg: &serde_json::Value) -> Option<String> {

        match msg.get("ch") {
            Some(e) => {
                return Some(msg.get("ch").unwrap().to_string());
            },
            None => {
                return None;
            }
        };
    }

}
