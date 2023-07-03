use core::fmt::Debug;
use serde::{Deserialize, Serialize};
use erased_serde::serialize_trait_object;
use serde_this_or_that::{as_f64};


// ---

serialize_trait_object!(Json);


// pub enum ExchangeJson {
//     EmptyJson,
//     TradeJson,
//     TickerJson
// }

pub trait Json: erased_serde::Serialize {}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmptyJson {}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeSingletonJson {

    #[serde(rename(serialize="amount", deserialize="amount"), deserialize_with="as_f64")]
    amount: f64,

    #[serde(rename(serialize="direction", deserialize="direction"))]
    direction: String,

    #[serde(rename(serialize="id", deserialize="id"), deserialize_with="as_f64")]
    id: f64,

    #[serde(rename(serialize="price", deserialize="price"), deserialize_with="as_f64")]
    price: f64,

    #[serde(rename(serialize="trade_id", deserialize="tradeId"))]
    trade_id: u64,

    #[serde(rename(serialize="trade_timestamp_utc", deserialize="ts"))]
    trade_timestamp_utc: u64,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeJson {
    trades: Vec<TradeSingletonJson>
}

impl TradeJson {
    pub fn new(trades: Vec<TradeSingletonJson>) -> TradeJson {
        TradeJson{trades: trades}
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TickerJson {
        #[serde(rename(serialize="high_24h", deserialize="amount"), deserialize_with="as_f64")]
        high_24h: f64,
        #[serde(rename(serialize="ask_price", deserialize="ask"), deserialize_with="as_f64")]
        ask_price: f64,
        #[serde(rename(serialize="ask_quantity", deserialize="askSize"), deserialize_with="as_f64")]
        ask_quantity: f64,
        #[serde(rename(serialize="bid_price", deserialize="bid"), deserialize_with="as_f64")]
        bid_price: f64,
        #[serde(rename(serialize="bid_quantity", deserialize="bidSize"), deserialize_with="as_f64")]
        bid_quantity: f64,
        #[serde(rename(serialize="close", deserialize="close"), deserialize_with="as_f64")]
        close: f64,
        #[serde(rename(serialize="count", deserialize="count"))]
        count: u32,
        #[serde(rename(serialize="high", deserialize="high"), deserialize_with="as_f64")]
        high: f64,
        #[serde(rename(serialize="last_price", deserialize="lastPrice"), deserialize_with="as_f64")]
        last_price: f64,
        #[serde(rename(serialize="last_quantity", deserialize="lastSize"), deserialize_with="as_f64")]
        last_quantity: f64,
        #[serde(rename(serialize="low", deserialize="low"), deserialize_with="as_f64")]
        low: f64,
        #[serde(rename(serialize="open", deserialize="open"), deserialize_with="as_f64")]
        open: f64,
        #[serde(rename(serialize="timestamp_utc", deserialize="ts"))]
        timestamp_utc: u64,
        #[serde(rename(serialize="volume", deserialize="vol"), deserialize_with="as_f64")]
        volume: f64,
}

impl Json for EmptyJson {}
impl Json for TickerJson {}
impl Json for TradeJson {}
impl Json for TradeSingletonJson {}

