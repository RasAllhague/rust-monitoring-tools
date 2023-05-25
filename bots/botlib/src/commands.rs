use async_trait::async_trait;
use monitoring_core::client::SysInfoClient;
use serenity::{
    builder::{CreateApplicationCommands},
    model::prelude::interaction::application_command::ApplicationCommandInteraction,
    prelude::Context,
};

use crate::error::CommandError;

#[async_trait]
pub trait SlashCommand: Send + Sync {
    type Config;

    fn register<'a>(
        &'a self,
        commands: &'a mut CreateApplicationCommands,
    ) -> &mut CreateApplicationCommands;

    async fn dispatch(
        &self,
        command: &ApplicationCommandInteraction,
        ctx: &Context,
        client: &SysInfoClient,
        config: &Self::Config,
    ) -> Result<(), CommandError>;
    fn name(&self) -> String;
}
