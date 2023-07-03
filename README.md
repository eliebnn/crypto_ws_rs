# crypto_ws_rs

Example on how to query Async Websocket Data from Binance, Bybit & Huobi.

To be developped further to match existing Python crypto_ws package.

**Note**: Currently, only 24hTicker and Public Trades are handled.

Using serde_json package, and existing matching structs, it extracts and parses data from the tungstenite::Message, Text or Binary, allowing downstream usage of data_str String: can be sent via Redis, Kafka, UDP, etc.

## Example

```rust
mod huobi;
use huobi::client::HuobiClient;


#[tokio::main]
async fn main() {

    let cls = HuobiClient::new();
    cls.run().await;

}
```