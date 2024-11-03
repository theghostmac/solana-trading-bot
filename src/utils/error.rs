use thiserror::Error;
use solana_client::client_error::ClientError;

#[derive(Error, Debug)]
pub enum BotError {
    #[error("Telegram API error: {0}")]
    TelegramError(#[from] teloxide::RequestError),

    #[error("Channel Send error: {0}")]
    ChannelSendError(String),

    #[error("Invalid Solana address format")]
    InvalidAddress,

    #[error("Solana client error: {0}")]
    SolanaClientError(#[from] ClientError),

    #[error("WebSocket connection error: {0}")]
    WebSocketError(String),

    #[error("Transaction analysis error: {0}")]
    TransactionAnalysisError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),
}

#[derive(Debug, Error)]
pub enum WalletError {
    #[error("Invalid wallet address provided: {0}")]
    InvalidAddress(String),
    #[error("RPC client error: {0}")]
    RpcError(#[from] ClientError),
    #[error("Lock acquisition failed")]
    LockError,
}