use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::stream::StreamExt;
use url::Url;
use futures_util::SinkExt;
use serde_json::{Value};

use super::messages::{TickerJson, TradeJson, EmptyJson, Json};
use super::payload::Payload;

pub struct BinanceClient {
    payload: Payload,
    url: String,
    ping_delay: u8,
}

impl BinanceClient {

    pub fn new() -> BinanceClient {
        BinanceClient {
            payload: Payload::new(),
            url: String::from("wss://stream.binance.com:9443/ws"),
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

            channel.clear();

            if let Some(tmp) = BinanceClient::parse_channel(&raw_msg) {
                channel = tmp;
            } else {
                continue 'main_loop;
            }

            let msg_text = &raw_msg.to_text().unwrap();
            let data: Box<dyn Json> = 
            
            if channel.contains("24hrTicker") {
                
                match serde_json::from_str::<TickerJson>(msg_text) {
                    Result::Ok(val) => Box::new(val),
                    Result::Err(err) => {println!("Error: {}", err); continue 'main_loop;}
                }
            }

            else if channel.contains("trade") {
                
                match serde_json::from_str::<TradeJson>(msg_text) {
                    Result::Ok(val) => {Box::new(val)},
                    Result::Err(err) => {println!("Error: {}", err); continue 'main_loop;}
                }
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
                match a.get("e") {
                    Some(_e) => {
                        return Some(a.get("e").unwrap().to_string());
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
