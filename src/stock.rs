use yahoo_finance_api::{self as yahoo};
use tokio_test;

pub fn fetch_stock_data() -> String {
    let provider = yahoo::YahooConnector::new().unwrap();
    // get the latest quotes in 1 minute intervals
    let response = tokio_test::block_on(provider.get_latest_quotes("0700.HK", "1d")).unwrap();
    // extract just the latest valid quote summery
    // including timestamp,open,close,high,low,volume
    let quote = response.last_quote().unwrap();
    return  format!("0700.HK close: {}", quote.close);
}