mod config;
mod telegram;
mod trading;
mod utils;

use config::environment::Environment;
use telegram::bot::start_bot;

#[tokio::main]
async fn main() {
    // Initialize configuration
    let env = Environment::new();

    // Start the Telegram bot
    if let Err(e) = start_bot(&env).await {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}