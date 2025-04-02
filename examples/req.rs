use yahoo_finance_api as yahoo;
use tokio_test;

fn main() {
    let provider = yahoo::YahooConnector::new().unwrap();
    // get the latest quotes in 1 minute intervals
    let response = tokio_test::block_on(provider.get_latest_quotes("0700.HK", "1d")).unwrap();
    // extract just the latest valid quote summery
    // including timestamp,open,close,high,low,volume
    let quote = response.last_quote().unwrap();

    println!("At {:?}", quote);
}