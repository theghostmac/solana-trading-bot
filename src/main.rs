mod config;
mod telegram;
mod trading;
mod utils;

use config::environment::Environment;
use telegram::bot::start_bot;
use crate::trading::core::TradingEngine;

#[tokio::main]
async fn main() {
    // Initialize configuration
    let env = Environment::new();

    // Initialize the trading engine.
    let trading_engine = TradingEngine::new(&env);

    // Clone the trading engine to make sure it lives long enough for both tasks
    let trading_engine_clone = trading_engine.clone();

    // Start the trading engine and Telegram bot concurrently
    tokio::select! {
        _ = trading_engine_clone.run() => {
            eprintln!("Trading engine has stopped");
        }
        result = start_bot(&env, trading_engine) => {
            if let Err(e) = result {
                eprintln!("Application error: {}", e);
                std::process::exit(1);
            }
        }
    }
}