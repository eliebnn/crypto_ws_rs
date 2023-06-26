mod binance;
use binance::client::BinanceClient;

// mod huobi;
// use huobi::client::HuobiClient;


#[tokio::main]
async fn main() {

    let cls = BinanceClient::new();
    cls.run().await;

}
