use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::collections::HashSet;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::config::environment::Environment;
use crate::utils::error::{BotError, WalletError};
use std::time::Duration;
use futures::TryFutureExt;

/// Tracks Solana wallet addresses and their states
///
/// The `WalletTracker` maintains a thread-safe set of wallet addresses
/// and provides methods to manage wallet tracking operations.
///
/// # Thread Safety
/// All operations are protected by a Mutex, making them safe for concurrent access.
pub struct WalletTracker {
    pub rpc_client: RpcClient,
    pub tracked_wallets: Arc<Mutex<HashSet<Pubkey>>>,
    env: Environment,
}

impl WalletTracker {
    /// Creates a new instance of WalletTracker
    ///
    /// # Arguments
    /// * `env` - Environment configuration containing RPC URL and other settings
    ///
    /// # Returns
    /// Arc-wrapped WalletTracker instance for thread-safe sharing
    pub fn new(env: &Environment) -> Arc<Self> {
        Arc::new(Self {
            rpc_client: RpcClient::new(env.solana_rpc_url.clone()),
            tracked_wallets: Arc::new(Mutex::new(HashSet::new())),
            env: env.clone(),
        })
    }

    /// Adds a wallet address to the tracking set
    ///
    /// # Arguments
    /// * `address` - Solana wallet address as a string
    ///
    /// # Returns
    /// * `Ok(())` if the wallet was successfully added
    /// * `Err(WalletError)` if the address is invalid or lock acquisition fails
    ///
    /// # Example
    /// ```rust
    /// # use your_crate::{WalletTracker, Environment};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let env = Environment::new();
    /// let tracker = WalletTracker::new(&env);
    /// tracker.add_wallet("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn add_wallet(&self, address: &str) -> Result<(), BotError> {
        let pubkey = Pubkey::from_str(address).map_err(|_| BotError::InvalidAddress)?;
        let mut wallets = self.tracked_wallets.lock().await;
        wallets.insert(pubkey);
        Ok(())
    }

    /// Removes a wallet address from the tracking set
    ///
    /// # Arguments
    /// * `address` - Solana wallet address as a string
    ///
    /// # Returns
    /// * `Ok(())` if the wallet was successfully removed
    /// * `Err(WalletError)` if the address is invalid or lock acquisition fails
    pub async fn remove_wallet(&self, address: &str) -> Result<(), BotError> {
        let pubkey = Pubkey::from_str(address).map_err(|_| BotError::InvalidAddress)?;
        let mut wallets = self.tracked_wallets.lock().await;
        wallets.remove(&pubkey);
        Ok(())
    }

    /// Checks if a wallet is currently being tracked
    ///
    /// # Arguments
    /// * `address` - Solana wallet address as a string
    ///
    /// # Returns
    /// * `Ok(bool)` indicating if the wallet is tracked
    /// * `Err(WalletError)` if the address is invalid or lock acquisition fails
    pub async fn is_tracked(&self, address: &str) -> Result<bool, WalletError> {
        let pubkey = Pubkey::from_str(address)
            .map_err(|_| WalletError::InvalidAddress(address.to_string()))?;

        let wallets = self.tracked_wallets
            .lock()
            .await
            .map_err(|_| WalletError::LockError)?;

        Ok(wallets.contains(&pubkey))
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use mockall::predicate::*;
//
//     // Mock environment for testing.
//     #[derive(Clone)]
//     struct TestEnvironment {
//         solana_rpc_url: String,
//     }
//
//     impl Environment for TestEnvironment {
//         fn new() -> Self {
//             Self {
//                 solana_rpc_url: "http://localhost:8899".to_string(),
//             }
//         }
//     }
//
//     #[tokio::test]
//     async fn test_add_wallet_success() {
//         let env = TestEnvironment::new();
//         let tracker = WalletTracker::new(&env);
//
//         let result = tracker
//             .add_wallet("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v")
//             .await;
//
//         assert!(result.is_ok());
//     }
//
//     #[tokio::test]
//     async fn test_add_wallet_invalid_address() {
//         let env = TestEnvironment::new();
//         let tracker = WalletTracker::new(&env);
//
//         let result = tracker.add_wallet("invalid_address").await;
//
//         assert!(matches!(result, Err(WalletError::InvalidAddress(_))));
//     }
//
//     #[tokio::test]
//     async fn test_remove_wallet_success() {
//         let env = TestEnvironment::new();
//         let tracker = WalletTracker::new(&env);
//         let address = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
//
//         // First add the wallet
//         tracker.add_wallet(address).await.unwrap();
//
//         // Then remove it
//         let result = tracker.remove_wallet(address).await;
//
//         assert!(result.is_ok());
//         assert!(!tracker.is_tracked(address).await.unwrap());
//     }
//
//     #[tokio::test]
//     async fn test_is_tracked() {
//         let env = TestEnvironment::new();
//         let tracker = WalletTracker::new(&env);
//         let address = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
//
//         // Initially should not be tracked
//         assert!(!tracker.is_tracked(address).await.unwrap());
//
//         // Add wallet
//         tracker.add_wallet(address).await.unwrap();
//
//         // Should now be tracked
//         assert!(tracker.is_tracked(address).await.unwrap());
//     }
// }