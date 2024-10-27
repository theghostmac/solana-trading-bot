mod config;
mod telegram;
mod trading;
mod utils;

use tokio::sync::mpsc;

use config::environment::Environment;
use telegram::bot::start_bot;
use trading::wallet_tracker::WalletTracker;
use crate::trading::core::TradingEngine;

#[tokio::main]
async fn main() {
    // Initialize configuration
    let env = Environment::new();

    // Initialize the trading engine.
    let trading_engine = TradingEngine::new(&env);
    
    // Initialize the wallet tracker.
    let wallet_tracker = WalletTracker::new(&env);
    
    // Create a channel for communication between components.
    let (tx, mut rx) = mpsc::channel(100);
    

    // Clone the components for the tasks.
    let trading_engine_clone = trading_engine.clone();
    let wallet_tracker_clone = wallet_tracker.clone();
    let env_clone = env.clone();

    // Start the trading engine, wallet tracker, and Telegram bot concurrently.
    tokio::select! {
        _ = trading_engine_clone.run() => {
            eprintln!("Trading engine has stopped");
        },
        _ = wallet_tracker_clone.start_tracking(tx) => {
            eprintln!("Wallet tracker has stopped");
        },
        result = start_bot(&env, trading_engine, wallet_tracker) => {
            if let Err(e) = result {
                eprintln!("Application error: {}", e);
                std::process::exit(1);
            }
        }
        _ = async move {
            while let Some(message) = rx.recv().await {
                println!("Received message: {}", message);
                // Send a message.
            }
        } => {
            eprintln!("Message receiver has stopped");
        }
    }
}