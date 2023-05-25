use clap::{Parser, Subcommand};
use monitoring_core::client::ClientConfig;
use serde::{Deserialize, Serialize};
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

use crate::error::CliError;

#[derive(Parser)]
#[command(name = "monitoring-cli")]
#[command(
    about = "Commandline tool for retrieving and sending system information for monitoring purposes."
)]
#[command(version = "0.1.0")]
#[command(author, long_about = None)]
pub enum Cli {
    #[command(subcommand, about = "Runs the cli.")]
    Run(RunOpt),
    #[command(about = "Configures the cli.")]
    Configure {
        #[arg(short, long, help = "Sets the api key in the configuration file.")]
        api_key: Option<String>,
        #[arg(
            short,
            long,
            help = "Sets the device profile key in the configuration file."
        )]
        profile_key: Option<String>,
        #[arg(
            short = 'i',
            long = "id",
            help = "Sets the device profile id in the configuration file."
        )]
        profile_id: Option<u32>,
        #[arg(short, long, help = "Sets the server url in the configuration file.")]
        server_url: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum RunOpt {
    Normal {
        #[arg(short, long, help = "Includes/Excludes cpu infos.")]
        cpu: bool,
        #[arg(short, long, help = "Includes/Excludes memory infos.")]
        memory: bool,
        #[arg(short, long, help = "Includes/Excludes os infos.")]
        os: bool,
        #[arg(short, long, help = "Includes/Excludes network infos.")]
        network: bool,
        #[arg(short, long, help = "Includes/Excludes filesystem infos.")]
        filesystem: bool,
        #[arg(short, long, help = "Includes/Excludes swap infos.")]
        swap: bool,
    },
    Service {
        #[arg(short, long, help = "Includes/Excludes cpu infos.")]
        cpu: bool,
        #[arg(short, long, help = "Includes/Excludes memory infos.")]
        memory: bool,
        #[arg(short, long, help = "Includes/Excludes os infos.")]
        os: bool,
        #[arg(short, long, help = "Includes/Excludes network infos.")]
        network: bool,
        #[arg(short, long, help = "Includes/Excludes filesystem infos.")]
        filesystem: bool,
        #[arg(short, long, help = "Includes/Excludes swap infos.")]
        swap: bool,
        #[arg(short = 'i', long = "interval", help = "Sleep interval in seconds.")]
        sleep_interval: u64,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CliConfig {
    pub client: ClientConfig,
}

impl CliConfig {
    pub async fn load(file_path: &str) -> Result<Self, CliError> {
        let contents = fs::read_to_string(file_path)
            .await
            .map_err(|x| CliError::Io(x))?;
        let config =
            serde_json::from_str::<CliConfig>(&contents).map_err(|x| CliError::Serde(x))?;

        Ok(config)
    }

    pub async fn save(&self, file_path: &str) -> Result<(), CliError> {
        let contents = serde_json::to_string(self).map_err(|x| CliError::Serde(x))?;

        let mut file = File::create(file_path).await.map_err(|x| CliError::Io(x))?;
        file.write_all(contents.as_bytes())
            .await
            .map_err(|x| CliError::Io(x))?;

        Ok(())
    }
}
