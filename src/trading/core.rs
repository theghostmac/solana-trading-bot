use crate::config::environment::Environment;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct TradingEngine {
    env: Environment,
    is_trading: Arc<Mutex<bool>>,
}

impl TradingEngine {
    pub fn new(env: &Environment) -> Arc<Self> {
        Arc::new(Self {
            env: env.clone(),
            is_trading: Arc::new(Mutex::new(false)),
        })
    }

    pub async fn run(&self) {
        loop {
            let is_trading = *self.is_trading.lock().await;
            if is_trading {
                // TODO: Implement trading logic here
                println!("Trading...");
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    }

    pub async fn start_trading(&self) {
        let mut is_trading = self.is_trading.lock().await;
        *is_trading = true;
    }

    pub async fn stop_trading(&self) {
        let mut is_trading = self.is_trading.lock().await;
        *is_trading = false;
    }
}