
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BotError {
    #[error("Telegram API error: {0}")]
    TelegramError(#[from] teloxide::RequestError),
}