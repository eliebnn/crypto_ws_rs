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
    #[serde(rename(serialize="trade_id", deserialize="i"))]
    trade_id: String,
    #[serde(rename(serialize="trade_timestamp_utc", deserialize="T"))]
    trade_timestamp_utc: u64,
    #[serde(rename(serialize="trade_price", deserialize="p"), deserialize_with="as_f64")]
    trade_price: f64,
    #[serde(rename(serialize="trade_size", deserialize="v"), deserialize_with="as_f64")]
    trade_size: f64,
    #[serde(rename(serialize="direction", deserialize="S"))]
    direction: String,
    #[serde(rename(serialize="symbol", deserialize="s"))]
    symbol: String,
    #[serde(rename(serialize="is_block_trade", deserialize="BT"))]
    is_block_trade: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeJson {
    pub trades: Vec<TradeSingletonJson>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TickerJson {
        #[serde(rename(serialize="high_24h", deserialize="highPrice24h"), deserialize_with="as_f64")]
        high_24h: f64,
        #[serde(rename(serialize="close_price", deserialize="lastPrice"), deserialize_with="as_f64")]
        close_price: f64,
        #[serde(rename(serialize="low_24h", deserialize="lowPrice24h"), deserialize_with="as_f64")]
        low_24h: f64,
        #[serde(rename(serialize="close_price_24h", deserialize="prevPrice24h"), deserialize_with="as_f64")]
        close_price_24h: f64,
        #[serde(rename(serialize="price_change_pct_24h", deserialize="price24hPcnt"), deserialize_with="as_f64")]
        price_change_pct_24h: f64,
        #[serde(rename(serialize="symbol", deserialize="symbol"))]
        symbol: String,
        #[serde(rename(serialize="quote_volume_24h", deserialize="turnover24h"), deserialize_with="as_f64")]
        quote_volume_24h: f64,
        #[serde(rename(serialize="usd_price_index", deserialize="usdIndexPrice"), deserialize_with="as_f64")]
        usd_price_index: f64,
        #[serde(rename(serialize="volume_24h", deserialize="volume24h"), deserialize_with="as_f64")]
        volume_24h: f64,
}

impl Json for EmptyJson {}
impl Json for TickerJson {}
impl Json for TradeJson {}
impl Json for TradeSingletonJson {}

