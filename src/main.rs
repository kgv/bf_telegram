use std::error::Error;
use teloxide::{commands_repl, prelude::*, utils::command::BotCommands};
use tracing::{error, info, warn};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_ansi(false).init();
    info!("Starting bot...");
    let bot = Bot::from_env().auto_send();
    commands_repl(bot, answer, Command::ty()).await;
}

async fn answer(
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help => {
            info!("Help info");
            bot.send_message(message.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::Username(username) => {
            warn!("Username warn");
            bot.send_message(message.chat.id, format!("Your username is @{username}."))
                .await?;
        }
        Command::Version => {
            error!("Version error");
            bot.send_message(message.chat.id, env!("CARGO_PKG_VERSION"))
                .await?;
        }
    }
    Ok(())
}

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a username.")]
    Username(String),
    #[command(description = "display the version.")]
    Version,
}
