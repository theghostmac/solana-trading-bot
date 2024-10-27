use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::collections::HashSet;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::config::environment::Environment;
use crate::utils::error::BotError;
use std::time::Duration;

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
        loop {
            let wallets = self.tracked_wallets.lock().await.clone();
            if wallets.is_empty() {
                tokio::time::sleep(Duration::from_secs(10)).await;
                continue;
            }

            // For now, let's poll the wallets every few seconds
            // This is a temporary solution until we implement proper WebSocket handling.
            // The solana-client-helpers::WsClientBuilder wasn't working so...
            for wallet in wallets.iter() {
                match self.rpc_client.get_signatures_for_address(wallet) {
                    Ok(signatures) => {
                        for sig_info in signatures {
                            let message = format!("New transaction for {}: {:?}", wallet, sig_info.signature);
                            if let Err(e) = tx.send(message).await {
                                eprintln!("Failed to send notification: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error fetching signatures for {}: {}", wallet, e);
                    }
                }
            }

            // Sleep for a few seconds before the next poll
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }

    // TODO: Implement WebSocket-based tracking in the future
    async fn _subscribe_to_wallet_transactions(&self, _wallet: &Pubkey) -> Result<(), BotError> {
        Ok(())
    }
}