// mod binance;
// use binance::client::BinanceClient;

// mod bybit;
// use bybit::client::BybitClient;

mod huobi;
use huobi::client::HuobiClient;


#[tokio::main]
async fn main() {

    let cls = HuobiClient::new();
    cls.run().await;

}
