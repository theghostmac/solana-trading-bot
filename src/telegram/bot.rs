use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;
use crate::config::environment::Environment;
use crate::telegram::commands::Command;
use crate::utils::error::BotError;

pub async fn start_bot(env: &Environment) -> Result<(), BotError> {
    let bot = Bot::new(&env.telegram_bot_token);

    teloxide::repl(bot, |bot: Bot, msg: Message, cmd: Command| async move {
        answer(bot, msg, cmd).await
    })
        .await;

    Ok(())
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Start => {
            bot.send_message(msg.chat.id, "Welcome to the Solana Degen Bot!").await?;
        },
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions()).await?;
        },
    }

    Ok(())
}