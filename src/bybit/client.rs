use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::stream::StreamExt;
use url::Url;
use futures_util::SinkExt;
use serde_json::{Value};

use super::messages::{TickerJson, TradeJson, TradeSingletonJson, EmptyJson, Json};
use super::payload::Payload;

pub struct BybitClient {
    payload: Payload,
    url: String,
    ping_delay: u8,
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

impl BybitClient {

    pub fn new() -> BybitClient {
        BybitClient {
            payload: Payload::new(),
            url: String::from("wss://stream.bybit.com/v5/public/spot"),
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
        println!("{}", &payload);
        let mut channel: String = String::new();

        write.send(Message::Text(payload)).await.unwrap();
        
        'main_loop: 
        while let Some(Ok(raw_msg)) = read.next().await {

            channel.clear();

            if let Some(tmp) = BybitClient::parse_channel(&raw_msg) {
                channel = tmp;
            } else {
                continue 'main_loop;
            }

            let msg_dct = serde_json::from_str::<serde_json::Value>(&raw_msg.to_text().unwrap()).unwrap();
            let data_text = serde_json::to_string::<serde_json::Value>(&msg_dct.get(&"data").unwrap()).unwrap();

            let data: Box<dyn Json> = 
            
                if channel.contains("tickers") {

                    match serde_json::from_str::<TickerJson>(&data_text) {
                        Result::Ok(val) => Box::new(val),
                        Result::Err(err) => {println!("Error: {}", err); continue 'main_loop;}
                    }
                }

                else if channel.contains("publicTrade") {

                    let mut singleton_vec: Vec<TradeSingletonJson> = vec![];

                    'trade_loop:
                    for parsed_trade in (&msg_dct.get(&"data").unwrap().as_array().unwrap()).iter(){

                        let string_trade = serde_json::to_string::<serde_json::Value>(parsed_trade).unwrap();

                        match serde_json::from_str::<TradeSingletonJson>(&string_trade) {
                            Result::Ok(value) => {singleton_vec.push(value)},
                            Result::Err(err) => {println!("Error: {}", err); continue 'trade_loop;}
                        }

                    };

                    Box::new(TradeJson{trades: singleton_vec})
                }

                else {
                    Box::new(EmptyJson{})
                };

            let data_str = serde_json::to_string_pretty(&data).unwrap();
            println!("{}: {}", &channel, data_str);

        }
    }

    fn parse_channel(msg: &tungstenite::protocol::Message) -> Option<String> {

        match serde_json::from_str::<Value>(&msg.to_text().unwrap()) {

            Ok(a) => {
                match a.get("topic") {
                    Some(e) => {
                        let tmp = a.get("topic").unwrap().to_string();
                        let mut tmp = tmp.split(".").collect::<Vec<&str>>();
                        let _ = &tmp.pop();

                        return Some(tmp.join("."));
                    },
                    None => {
                        return None;
                    }
                };
            },
            Err(err) => {
                println!("Error while parsing {}", &err);
            }
        };
    
        None
    }


}
