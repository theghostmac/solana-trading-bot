use teloxide::macros::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "Start the bot.")]
    Start,
    #[command(description = "Show this help message.")]
    Help,
    #[command(description = "Start trading.")]
    StartTrading,
    #[command(description = "Stop trading.")]
    StopTrading,
}