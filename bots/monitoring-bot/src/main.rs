use std::sync::Arc;

use botlib::{commands::SlashCommand, handler::BotHandler};
use commands::profile::ProfileCommand;
use config::{AppConfigurations, BotConfig};
use serenity::{prelude::GatewayIntents, Client};
use tracing::{instrument, log::error};

mod config;
mod commands;

#[tokio::main]
#[instrument]
async fn main() {
    tracing_subscriber::fmt::init();

    let app_config = AppConfigurations::from_env();
    let bot_config = BotConfig::new(&app_config.api_token, &app_config.read_token, &app_config.server_url, &app_config.allowed_users);

    let mut commands: Vec<Arc<dyn SlashCommand<Config = BotConfig>>> = Vec::new();
    commands.push(Arc::new(ProfileCommand));

    let intents = GatewayIntents::default() | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MESSAGES;
    let mut client = Client::builder(app_config.bot_token, intents)
        .event_handler(BotHandler::new(&commands, bot_config))
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
