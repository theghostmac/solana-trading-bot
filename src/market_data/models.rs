use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TokenInfo {
    pub address: String,
    pub name: String,
    pub symbol: String,
}

#[derive(Deserialize, Debug)]
pub struct TxnCount {
    pub buys: u32,
    pub sells: u32,
}

#[derive(Deserialize, Debug)]
pub struct Volume {
    pub h24: f64,
    pub h6: f64,
    pub h1: f64,
    pub m5: f64,
}

#[derive(Deserialize, Debug)]
pub struct PriceChange {
    pub m5: f64,
    pub h1: f64,
    pub h6: f64,
    pub h24: f64,
}

#[derive(Deserialize, Debug)]
pub struct Liquidity {
    pub usd: f64,
    pub base: f64,
    pub quote: f64,
}

#[derive(Deserialize, Debug)]
pub struct PairData {
    pub chainId: String,
    pub dexId: String,
    pub url: String,
    pub pairAddress: String,
    pub baseToken: TokenInfo,
    pub quoteToken: TokenInfo,
    pub priceNative: String,
    pub priceUsd: String,
    pub txns: Txns,
    pub volume: Volume,
    pub priceChange: PriceChange,
    pub liquidity: Liquidity,
    pub fdv: f64,
    pub pairCreatedAt: u64,
}

#[derive(Deserialize, Debug)]
pub struct Txns {
    pub m5: TxnCount,
    pub h1: TxnCount,
    pub h6: TxnCount,
    pub h24: TxnCount,
}

#[derive(Deserialize, Debug)]
pub struct MarketData {
    pub pairs: Vec<PairData>,
}
