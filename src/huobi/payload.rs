use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Payload {
    method: String,
    params: Vec<String>,
}


impl Payload {
    pub fn new() -> Payload {
        Payload{
            method: "subscribe".to_string(),
            params: vec!["market.btcusdt.trade.detail".to_string()],
        }
    }

    pub fn to_string(&self) -> String {
        let p = self.params.join("\", \"");
        format!("{{\"sub\": \"{}\"}}", p)
    }
}
