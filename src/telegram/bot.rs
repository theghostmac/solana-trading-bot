use teloxide::prelude::*;
use crate::config::environment::Environment;
use crate::telegram::commands::Command;
use crate::trading::core::TradingEngine;
use crate::trading::wallet_tracker::WalletTracker;
use crate::utils::error::BotError;
use std::sync::Arc;
use teloxide::utils::command::BotCommands;

pub async fn start_bot(
    env: &Environment,
    trading_engine: Arc<TradingEngine>,
    wallet_tracker: Arc<WalletTracker>,
) -> Result<(), BotError> {
    let bot = Bot::new(&env.telegram_bot_token);

    Command::repl(bot, move |bot: Bot, msg: Message, cmd: Command| {
        let trading_engine = Arc::clone(&trading_engine);
        let wallet_tracker = Arc::clone(&wallet_tracker);
        async move {
            answer(bot, msg, cmd, trading_engine, wallet_tracker).await
        }
    })
        .await;

    Ok(())
}

async fn answer(
    bot: Bot,
    msg: Message,
    cmd: Command,
    trading_engine: Arc<TradingEngine>,
    wallet_tracker: Arc<WalletTracker>,
) -> ResponseResult<()> {
    match cmd {
        Command::Start => {
            bot.send_message(msg.chat.id, "Welcome to the Solana Trading Bot!").await?;
        },
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
        },
        Command::StartTrading => {
            trading_engine.start_trading().await;
            bot.send_message(msg.chat.id, "Trading started!").await?;
        },
        Command::StopTrading => {
            trading_engine.stop_trading().await;
            bot.send_message(msg.chat.id, "Trading stopped!").await?;
        },
        Command::AddWallet(address) => {
            match wallet_tracker.add_wallet(&address).await {
                Ok(_) => {
                    bot.send_message(msg.chat.id, format!("Started tracking wallet: {}", address)).await?;
                },
                Err(e) => {
                    bot.send_message(msg.chat.id, format!("Failed to add wallet: {}", e)).await?;
                }
            }
        },
        Command::RemoveWallet(address) => {
            match wallet_tracker.remove_wallet(&address).await {
                Ok(_) => {
                    bot.send_message(msg.chat.id, format!("Stopped tracking wallet: {}", address)).await?;
                },
                Err(e) => {
                    bot.send_message(msg.chat.id, format!("Failed to remove wallet: {}", e)).await?;
                }
            }
        },
    }

    Ok(())
}