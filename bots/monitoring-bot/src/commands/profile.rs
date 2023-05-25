use std::path::PathBuf;

use async_trait::async_trait;
use botlib::{commands::SlashCommand, error::CommandError};
use rand::Rng;
use serenity::{
    builder::{CreateApplicationCommand, CreateApplicationCommands},
    model::prelude::interaction::{
        application_command::ApplicationCommandInteraction, InteractionResponseType,
    },
    prelude::Context,
};
use tokio::fs::read_dir;
use tracing::log::info;

use crate::config::BotConfig;

static COMMAND_NAME: &str = "profile";

pub struct LssCommand;

#[async_trait]
impl SlashCommand for LssCommand {
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
        todo!()
    }

    fn name(&self) -> String {
        String::from(COMMAND_NAME)
    }
}

impl LssCommand {
    fn build(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .name(COMMAND_NAME)
            .description("Command for sending lls images.")
    }
}
