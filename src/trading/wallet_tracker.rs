use solana_client::{
    rpc_client::RpcClient,
    rpc_config::RpcTransactionLogsConfig,
    rpc_config::RpcTransactionLogsFilter,
};
use solana_sdk::pubkey::Pubkey;
use std::collections::HashSet;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::config::environment::Environment;
use crate::utils::error::BotError;
use solana_client_helpers::WsClientBuilder;
use futures::StreamExt;

pub struct WalletTracker {
    pub rpc_client: RpcClient,
    pub tracked_wallets: Arc<Mutex<HashSet<Pubkey>>>,
    env: Environment,
}

impl WalletTracker {
    pub fn new(env: &Environment) -> Arc<Self> {
        Arc::new(Self {
            rpc_client: RpcClient::new(env.solana_rpc_url.clone()),
            tracked_wallets: Arc::new(Mutex::new(HashSet::new())),
            env: env.clone(),
        })
    }

    pub async fn add_wallet(&self, address: &str) -> Result<(), BotError> {
        let pubkey = Pubkey::from_str(address).map_err(|_| BotError::InvalidAddress)?;
        let mut wallets = self.tracked_wallets.lock().await;
        wallets.insert(pubkey);
        Ok(())
    }
    
    pub async fn remove_wallet(&self, address: &str) -> Result<(), BotError> {
        let pubkey = Pubkey::from_str(address).map_err(|_| BotError::InvalidAddress)?;
        let mut wallets = self.tracked_wallets.lock().await;
        wallets.remove(&pubkey);
        Ok(())
    }
    
    pub async fn start_tracking(&self, tx: tokio::sync::mpsc::Sender<String>) -> Result<(), BotError> {
        let ws_url = self.env.solana_ws_url.clone();
        let mut ws_client = WsClientBuilder::new().build(&ws_url).await?;

        loop {
            let wallets = self.tracked_wallets.lock().await.clone();
            if wallets.is_empty() {
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                continue;
            }

            let subscription = ws_client.logs_subscribe(
                RpcTransactionLogsFilter::Mentions(wallets.iter().map(|w| w.to_string()).collect()),
                RpcTransactionLogsConfig { commitment: None },
            ).await?;

            let mut notifications = subscription.notifications();

            while let Some(log) = notifications.next().await {
                if let Ok(log_info) = log {
                    // Process the log and send a notification
                    let message = format!("New transaction: {:?}", log_info);
                    tx.send(message).await.map_err(|_| BotError::ChannelSendError)?;
                }
            }
        }
    }
}