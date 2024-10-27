use dotenv::dotenv;
use std::env;

#[derive(Clone)]
pub struct Environment {
    pub telegram_bot_token: String,
    pub solana_rpc_url: String,
    pub solana_ws_url: String,
}

impl Environment {
    pub fn new() -> Self {
        dotenv().ok();

        Environment {
            telegram_bot_token: env::var("TELEGRAM_BOT_TOKEN")
                .expect("TELEGRAM_BOT_TOKEN not set"),
            solana_rpc_url: env::var("SOLANA_RPC_URL")
                .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string()),
            solana_ws_url: env::var("SOLANA_WS_URL")
                .unwrap_or_else(|_| "wss://api.mainnet-beta.solana.com".to_string()),
        }
    }
}