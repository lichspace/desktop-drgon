use reqwest::Error;
use serde_json::Value;

pub async fn fetch_stock_data() -> Result<Value, Error> {
    let url = "https://query1.finance.yahoo.com/v7/finance/quote?fields=logoUrl%2CoptionsType%2CregularMarketSource%2CpostMarketTime%2CpostMarketPrice%2CpostMarketChange%2CpostMarketChangePercent%2CpreMarketTime%2CpreMarketPrice%2CpreMarketChange%2CpreMarketChangePercent%2CstockStory&formatted=true&imgHeights=50&imgLabels=logoUrl&imgWidths=50&symbols=0700.HK&enablePrivateCompany=true&lang=en-US&region=US&crumb=tWZ2VYP0p%2F1";
    println!("{}", url);

    // let response = reqwest::get(url).await?;
    // let text: String = response.text().await?;
    // let json: Value = serde_json::from_str(&text).unwrap();
    let json = serde_json::json!({
        "symbol": "0700.HK",
        "regularMarketPrice": 7.0,
        "regularMarketChange": 0.0,
        "regularMarketChangePercent": 0.0,
        "postMarketPrice": 7.0,
        "postMarketChange": 0.0,
        "postMarketChangePercent": 0.0,
        "preMarketPrice": 7.0,
    });
    return Ok(json);
}