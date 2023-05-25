use monitoring_core::client::SysInfoClient;
use serenity::{model::prelude::interaction::application_command::ApplicationCommandInteraction, prelude::Context};
use tracing::log::info;

use crate::error::CommandError;

pub struct CommandUsageLogger;

impl CommandUsageLogger {
    pub async fn log(
        ctx: &Context,
        interaction: &ApplicationCommandInteraction,
    ) -> Result<(), CommandError> {
        info!(
            "User {}:{} used the '{}' command!",
            interaction.user.id,
            interaction.guild_id.unwrap().0,
            interaction.data.name
        );

        Ok(())
    }
}