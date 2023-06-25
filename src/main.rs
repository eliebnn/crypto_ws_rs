// Importing required libraries for the application
use serde::{Deserialize, Serialize};
use serde_json::{Value};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tokio;
use url::Url;
use futures_util::stream::StreamExt;
use futures_util::SinkExt;

// Struct definition for a Payload that we will send via websocket
#[derive(Deserialize, Serialize)]
struct Payload {
    method: String,
    params: Vec<String>,
    id: u8
}

impl Payload {
    // Constructor for Payload with predefined values
    fn new() -> Payload {
        Payload{
            method: "SUBSCRIBE".to_string(),
            params: vec!["btcusdt@trade".to_string()],
            id: 1,
        }
    }

    // Converts the current Payload instance to a JSON-formatted String
    fn to_string(&self) -> String {
        let p = self.params.join("\", \"");
        format!("{{\"method\": \"{}\", \"params\": [\"{}\"], \"id\": {}}}", self.method, p, self.id)
    }
}

// Asynchronous main function
#[tokio::main]
async fn main() {

    // Attempt to connect to the server
    let (ws_stream, response) =
        connect_async(Url::parse("wss://stream.binance.com:9443/ws").unwrap()).await.expect("Can't connect");

    // Print connection success messages and response details
    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");

    // Loop through and print each header in the response
    for (ref header, val /* value */) in response.headers() {
        println!("* {}: {:?}", header, val);
    }

    // Split the websocket stream into write and read streams
    let (mut write, mut read) = ws_stream.split();

    // Create a new Payload instance and convert it to a JSON-formatted string
    let payload = Payload::new().to_string();

    // Send the Payload string to the server
    write.send(Message::Text(payload)).await.unwrap();
    
    // Infinite loop to keep listening to new messages from the server
    'main_loop: 
    while let Some(msg) = read.next().await {
        
        // Unwrap the received message
        let str_msg = msg.unwrap();
        println!("Received Message: {:?}", &str_msg);
            
        // Attempt to parse the received message into a serde_json::Value
        let parsed_msg = match serde_json::from_str::<serde_json::Value>(&str_msg.into_text().unwrap()) {
            Result::Ok(val) => val,
            Result::Err(err) => {
                // In case of parsing error, print error message and continue to the next iteration
                println!("Error while Parsing: {}", err);
                continue 'main_loop;
            }
        };
        
        // Print the parsed message
        println!("Parsed Message: {:?}", parsed_msg);
    }
}
