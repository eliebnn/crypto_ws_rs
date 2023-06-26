use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Payload {
    method: String,
    params: Vec<String>,
    id: u8
}


impl Payload {
    pub fn new() -> Payload {
        Payload{
            method: "SUBSCRIBE".to_string(),
            params: vec!["bnbusdt@ticker".to_string(), "bnbusdt@trade".to_string()],
            id: 1,
        }
    }

    pub fn to_string(&self) -> String {
        let p = self.params.join("\", \"");
        format!("{{\"method\": \"{}\", \"params\": [\"{}\"], \"id\": {}}}", self.method, p, self.id)
    }
}
