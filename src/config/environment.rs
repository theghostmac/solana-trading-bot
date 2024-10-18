use dotenv::dotenv;
use std::env;

pub struct Environment {
    pub telegram_bot_token: String,
}

impl Environment {
    pub fn new() -> Self {
        dotenv().ok();

        Environment {
            telegram_bot_token: env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set"),
        }
    }
}