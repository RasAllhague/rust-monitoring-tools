use std::path::PathBuf;

use async_trait::async_trait;
use botlib::{commands::SlashCommand, error::CommandError, parser::PositionalOptionParser};
use monitoring_core::{client::{SysInfoClient, ClientConfig}, api::models::InsertDeviceProfile};
use rand::Rng;
use serenity::{
    builder::{CreateApplicationCommand, CreateApplicationCommands},
    model::prelude::{interaction::{
        application_command::{ApplicationCommandInteraction, CommandDataOption}, InteractionResponseType,
    }, command::CommandOptionType},
    prelude::Context,
};
use tokio::fs::read_dir;
use tracing::log::{info, warn};

use crate::config::BotConfig;

static COMMAND_NAME: &str = "profile";

pub struct ProfileCommand;

#[async_trait]
impl SlashCommand for ProfileCommand {
    type Config = BotConfig;

    fn register<'a>(
        &'a self,
        commands: &'a mut CreateApplicationCommands,
    ) -> &mut CreateApplicationCommands {
        commands.create_application_command(|command| Self::build(command));

        commands
    }

    async fn dispatch(
        &self,
        command: &ApplicationCommandInteraction,
        ctx: &Context,
        config: &Self::Config,
    ) -> Result<(), CommandError> {
        command
            .create_interaction_response(ctx, |m| {
                m.kind(InteractionResponseType::DeferredChannelMessageWithSource)
            })
            .await?;

        if !config.allowed_users().iter().any(|x| *x == command.user.id.0) {
            command
                .edit_original_interaction_response(ctx, |m| {
                    m.content("You are not allowed to use this command!")
                })
                .await?;
    
            return Ok(());
        }

        match command.data.options[0].name.as_str() {
            "create" => run_create(command, ctx, config).await,
            "view" => run_view(command, ctx, config).await,
            _ => {
                warn!("Invalid command option found!");
                command
                    .edit_original_interaction_response(ctx, |m| {
                        m.content("Invalid command option!")
                    })
                    .await?;

                Ok(())
            },
        }
    }

    fn name(&self) -> String {
        String::from(COMMAND_NAME)
    }
}

impl ProfileCommand {
    fn build(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .name(COMMAND_NAME)
            .description("Commands for interacting with device profiles.")
            .create_option(|sub_command| {
                sub_command
                    .name("create")
                    .description("Command for creating a new profile.")
                    .kind(CommandOptionType::SubCommand)
                    .create_sub_option(|option| {
                        option
                            .name("device-name")
                            .description("The name of the device-profile.")
                            .kind(CommandOptionType::String)
                            .required(true)
                    })  
                    .create_sub_option(|option| {
                        option
                            .name("profile-key")
                            .description("The profile-key for this device profile.")
                            .kind(CommandOptionType::String)
                            .required(true)
                    })
            })
            .create_option(|sub_command| {
                sub_command
                    .name("view")
                    .description("Shows all device profiles.")
                    .kind(CommandOptionType::SubCommand)                    
            })
    }
}

pub async fn run_create(
    command: &ApplicationCommandInteraction,
    ctx: &Context,
    config: &BotConfig,
) -> Result<(), CommandError> {
    let client_config = ClientConfig::new(config.api_token(), config.server_url());
    let client = SysInfoClient::new(client_config);

    let device_name = PositionalOptionParser::parse_string(&command.data.options[0].options, 0)?;
    let profile_key = PositionalOptionParser::parse_string(&command.data.options[0].options, 1)?;

    let insert_profile = InsertDeviceProfile {
        device_name,
        profile_key,
        create_user: command.user.id.0 as i64,
    };

    client.post_profile(insert_profile).await?;

    command
        .edit_original_interaction_response(ctx, |m| {
            m.content("Created new device profile!")
        })
        .await?;

    Ok(())

}

pub async fn run_view(
    command: &ApplicationCommandInteraction,
    ctx: &Context,
    config: &BotConfig,
) -> Result<(), CommandError> {
    let client_config = ClientConfig::new(config.api_token(), config.server_url());
    let client = SysInfoClient::new(client_config);

    Ok(())
}