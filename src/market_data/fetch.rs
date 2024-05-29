use crate::market_data::models::MarketData;

pub async fn fetch_market_data(token_address: &str) -> Result<MarketData, reqwest::Error> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://api.dexscreener.io/latest/dex/tokens/{}",
        token_address
    );

    let resp = client.get(&url).send().await?;

    let market_data = resp.json::<MarketData>().await?;

    Ok(market_data)
}