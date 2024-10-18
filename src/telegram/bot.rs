use teloxide::prelude::*;
use crate::config::environment::Environment;
use crate::telegram::commands::Command;
use crate::trading::core::TradingEngine;
use crate::utils::error::BotError;
use std::sync::Arc;
use teloxide::utils::command::BotCommands;

pub async fn start_bot(env: &Environment, trading_engine: Arc<TradingEngine>) -> Result<(), BotError> {
    let bot = Bot::new(&env.telegram_bot_token);

    teloxide::repl(bot, move |bot: Bot, msg: Message, cmd: Command| {
        let trading_engine = Arc::clone(&trading_engine);
        async move {
            answer(bot, msg, cmd, trading_engine).await
        }
    })
        .await;

    Ok(())
}

async fn answer(bot: Bot, msg: Message, cmd: Command, trading_engine: Arc<TradingEngine>) -> ResponseResult<()> {
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
    }

    Ok(())
}