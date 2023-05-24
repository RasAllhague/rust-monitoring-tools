mod config;
mod error;
mod client;

use clap::Parser;
use client::SysInfoClient;
use config::{Cli, RunOpt, CliConfig, ClientConfig};
use error::CliError;
use monitoring_core::{models::SystemInformation, options::CollectorOptions};
use systemstat::{Duration, Platform, System};
use tokio::time::sleep;
use log::{warn, error, info};

pub static CONFIG_FILE_PATH: &str = "./config.json";

#[tokio::main]
async fn main() -> Result<(), CliError> {
    let cli = Cli::parse();
    env_logger::init();
    
    match cli {
        Cli::Run(run_opt) => run(run_opt).await,
        Cli::Configure { api_key, profile_key, profile_id, server_url } => 
            configure(api_key, profile_key, profile_id, server_url).await,
    }
}

async fn run(run_opt: RunOpt) -> Result<(), CliError> {
    match run_opt {
        RunOpt::Normal { cpu, memory, os, network, filesystem, swap } => 
            run_normal(CollectorOptions::new(cpu, memory, os, network, filesystem, swap)).await,
        RunOpt::Service { cpu, memory, os, network, filesystem, swap, sleep_interval } => 
            run_service(CollectorOptions::new(cpu, memory, os, network, filesystem, swap), sleep_interval).await,
    }
}

async fn run_normal(options: CollectorOptions) -> Result<(), CliError> {
    let cli_config = CliConfig::load(CONFIG_FILE_PATH).await?;

    post_system_info(&options, &cli_config.client).await
}

async fn run_service(options: CollectorOptions, sleep_interval: u64) -> Result<(), CliError> {
    let cli_config = CliConfig::load(CONFIG_FILE_PATH).await?;

    loop {
        post_system_info(&options, &cli_config.client).await?;

        sleep(Duration::from_secs(sleep_interval)).await;
    }
}

async fn post_system_info(options: &CollectorOptions, client_config: &ClientConfig) -> Result<(), CliError>  {
    let system = System::new();
    let hostname = hostname::get().map_err(|x| CliError::Io(x))?;
    let os_info = if options.os() {
        Some(os_info::get())
    }
    else {
        None
    };

    if let Ok(info) = SystemInformation::collect(options, system, hostname, os_info) {
        let client = SysInfoClient::new(client_config.clone());

        match client.get_version().await {
            Ok(version) => {
                info!("Server version: {version}");

                if let Err(why) = client.post_sys_info(info).await {
                    error!("Failed to post system information, error: {:?}", why);
                }
            },
            Err(why) => error!("Failed to get server version, error: {:?}", why),
        }
    }
    else {
        warn!("Collecting of system information failed!");
    }

    Ok(())
}

async fn configure(api_key: Option<String>, profile_key: Option<String>, profile_id: Option<u32>, server_url: Option<String>) -> Result<(), CliError> {
    let mut cli_config = CliConfig::load(CONFIG_FILE_PATH).await?;

    if let Some(api_key) = api_key {
        cli_config.client.api_key = api_key;
    }
    if let Some(profile_key) = profile_key {
        cli_config.client.profile_key = profile_key;
    }
    if let Some(profile_id) = profile_id {
        cli_config.client.profile_id = profile_id;
    }
    if let Some(server_url) = server_url {
        cli_config.client.server_url = server_url;
    }

    cli_config.save(CONFIG_FILE_PATH).await?;

    Ok(())
}