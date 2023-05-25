use std::sync::Arc;

use async_trait::async_trait;
use serenity::{prelude::{EventHandler, Context}, model::prelude::{ResumedEvent, Ready, command::Command, interaction::Interaction}};
use tracing::{log::{error, info, debug}, instrument};

use crate::{commands::SlashCommand, logger::CommandUsageLogger};

pub struct BotHandler<T: Clone + Sync + Send> {
    pub commands: Vec<Arc<dyn SlashCommand<Config = T>>>,
    pub config: T,
}

impl<T: Clone + Sync + Send> BotHandler<T> {
    pub fn new(
        commands: &[Arc<dyn SlashCommand<Config = T>>],
        config: T,
    ) -> Self {
        Self {
            commands: commands.into(),
            config,
        }
    }
}

#[async_trait]
impl<T: Clone + Sync + Send> EventHandler for BotHandler<T> {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command_interaction) = interaction {
            debug!("Received command interaction: {:#?}", command_interaction);

            if command_interaction.guild_id.is_none() {
                return;
            }

            for command in self
                .commands
                .iter()
                .filter(|x| x.name() == command_interaction.data.name)
            {
                if let Err(why) = CommandUsageLogger::log(&ctx, &command_interaction)
                    .await
                {
                    error!("Failed to log command usage: {:?}", why);
                }

                if let Err(why) = command
                    .dispatch(&command_interaction, &ctx, &self.config)
                    .await
                {
                    error!("Error during command interaction: {:?}", why);
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let guild_commands =
            Command::set_global_application_commands(&ctx.http, |command_builder| {
                for command in &self.commands {
                    command.register(command_builder);
                }

                command_builder
            })
            .await;

        if let Err(why) = guild_commands {
            error!("Failed to create slash commands. {}", why);
        }
    }

    #[instrument(skip(self, _ctx))]
    async fn resume(&self, _ctx: Context, resume: ResumedEvent) {
        debug!("Resumed; trace: {:?}", resume.trace);
    }
}