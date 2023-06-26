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
pub struct TradeJson {
    #[serde(rename(serialize="event_type", deserialize="e"))]
    event_type: String,
    #[serde(rename(serialize="event_time_utc", deserialize="E"))]
    event_time_utc: u64,
    #[serde(rename(serialize="symbol", deserialize="s"))]
    symbol: String,
    #[serde(rename(serialize="trade_id", deserialize="t"))]
    trade_id: u32,
    #[serde(rename(serialize="price", deserialize="p"), deserialize_with="as_f64")]
    price: f64,
    #[serde(rename(serialize="quantity", deserialize="q"), deserialize_with="as_f64")]
    quantity: f64,
    #[serde(rename(serialize="buy_order_id", deserialize="b"))]
    buy_order_id: u64,
    #[serde(rename(serialize="sell_order_id", deserialize="a"))]
    sell_order_id: u64,
    #[serde(rename(serialize="trade_time", deserialize="T"))]
    trade_time: u64,
    #[serde(rename(serialize="buyer_is_maker", deserialize="m"))]
    buyer_is_maker: bool,
    #[serde(rename(serialize="best_price_match", deserialize="M"))]
    best_price_match: bool
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TickerJson {
        #[serde(rename(serialize="event_type", deserialize="e"))]
        event_type: String,
        #[serde(rename(serialize="event_time_utc", deserialize="E"))]
        event_time_utc: u64,
        #[serde(rename(serialize="symbol", deserialize="s"))]
        symbol: String,
        #[serde(rename(serialize="price_change_24h", deserialize="p"), deserialize_with="as_f64")]
        price_change_24h: f64,
        #[serde(rename(serialize="price_change_pct_24h", deserialize="P"), deserialize_with="as_f64")]
        price_change_pct_24h: f64,
        #[serde(rename(serialize="wgt_avg_price_24h", deserialize="w"), deserialize_with="as_f64")]
        wgt_avg_price_24h: f64,
        #[serde(rename(serialize="close_price_24h", deserialize="x"), deserialize_with="as_f64")]
        close_price_24h: f64,
        #[serde(rename(serialize="last_price", deserialize="c"), deserialize_with="as_f64")]
        last_price: f64,
        #[serde(rename(serialize="bid_price", deserialize="b"), deserialize_with="as_f64")]
        bid_price: f64,
        #[serde(rename(serialize="bid_quantity", deserialize="B"), deserialize_with="as_f64")]
        bid_quantity: f64,
        #[serde(rename(serialize="ask_price", deserialize="a"), deserialize_with="as_f64")]
        ask_price: f64,
        #[serde(rename(serialize="ask_quantity", deserialize="A"), deserialize_with="as_f64")]
        ask_quantity: f64,
        #[serde(rename(serialize="open", deserialize="o"), deserialize_with="as_f64")]
        open: f64,
        #[serde(rename(serialize="high", deserialize="h"), deserialize_with="as_f64")]
        high: f64,
        #[serde(rename(serialize="low_24h", deserialize="l"), deserialize_with="as_f64")]
        low_24h: f64,
        #[serde(rename(serialize="volume_24h", deserialize="v"), deserialize_with="as_f64")]
        volume_24h: f64,
        #[serde(rename(serialize="quote_volume_24h", deserialize="q"), deserialize_with="as_f64")]
        quote_volume_24h: f64,
        #[serde(rename(serialize="Q", deserialize="Q"), deserialize_with="as_f64")]
        Q: f64,
        #[serde(rename(serialize="open_time_utc", deserialize="O"))]
        open_time_utc: u64,
        #[serde(rename(serialize="close_time_utc", deserialize="C"))]
        close_time_utc: u64,
        #[serde(rename(serialize="first_trade_id", deserialize="F"))]
        first_trade_id: u64,
        #[serde(rename(serialize="last_trade_id", deserialize="L"))]
        last_trade_id: u64,
        #[serde(rename(serialize="number_trades", deserialize="n"))]
        number_trades: u32
}

impl Json for EmptyJson {}
impl Json for TickerJson {}
impl Json for TradeJson {}
