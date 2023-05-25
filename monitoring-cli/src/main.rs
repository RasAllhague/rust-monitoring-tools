mod client;
mod config;
mod error;

use std::{path::Path, env};

use chrono::Local;
use clap::Parser;
use client::SysInfoClient;
use config::{Cli, CliConfig, ClientConfig, RunOpt};
use env_logger::Builder;
use error::CliError;
use log::{error, info, warn, LevelFilter};
use monitoring_core::{models::SystemInformation, options::CollectorOptions};
use systemstat::{Duration, Platform, System};
use tokio::time::sleep;
use std::io::Write;

pub static CONFIG_FILE_PATH: &str = "./config.json";

#[tokio::main]
async fn main() -> Result<(), CliError> {
    let cli = Cli::parse();
    
    Builder::new()
        .format(|buf, record| {
            writeln!(buf,
                "[{}] [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .parse_env(&env::var("MONITORING_CLI_LOG").unwrap_or_default())
        .filter(None, LevelFilter::Info)
        .init();

    match cli {
        Cli::Run(run_opt) => run(run_opt).await,
        Cli::Configure {
            api_key,
            profile_key,
            profile_id,
            server_url,
        } => configure(api_key, profile_key, profile_id, server_url).await,
    }
}

async fn run(run_opt: RunOpt) -> Result<(), CliError> {
    match run_opt {
        RunOpt::Normal {
            cpu,
            memory,
            os,
            network,
            filesystem,
            swap,
        } => {
            run_normal(CollectorOptions::new(
                cpu, memory, os, network, filesystem, swap,
            ))
            .await
        }
        RunOpt::Service {
            cpu,
            memory,
            os,
            network,
            filesystem,
            swap,
            sleep_interval,
        } => {
            run_service(
                CollectorOptions::new(cpu, memory, os, network, filesystem, swap),
                sleep_interval,
            )
            .await
        }
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

async fn post_system_info(
    options: &CollectorOptions,
    client_config: &ClientConfig,
) -> Result<(), CliError> {
    let system = System::new();
    let hostname = hostname::get().map_err(|x| CliError::Io(x))?;
    let os_info = if options.os() {
        Some(os_info::get())
    } else {
        None
    };

    if let Ok(info) = SystemInformation::collect(options, system, hostname, os_info) {
        let client = SysInfoClient::new(client_config.clone());

        match client.get_version().await {
            Ok((version, status)) => {
                if !status.is_success() {
                    error!("Failed to get server verion, status code: {status}");
                    return Ok(());
                }

                info!("Server version: {version}");

                if let Err(why) = client.post_sys_info(info).await {
                    error!("Failed to post system information, error: {:?}", why);
                }
                else {
                    info!("Successfully send system information for profile {}", client_config.profile_id);
                }
            }
            Err(why) => error!("Failed to get server version, error: {:?}", why),
        }
    } else {
        warn!("Collecting of system information failed!");
    }

    Ok(())
}

async fn configure(
    api_key: Option<String>,
    profile_key: Option<String>,
    profile_id: Option<u32>,
    server_url: Option<String>,
) -> Result<(), CliError> {
    if !Path::new(CONFIG_FILE_PATH).exists() {
        let new_config = CliConfig {
            client: ClientConfig { api_key: String::new(), profile_key: String::new(), profile_id: 0, server_url: String::new() }
        };

        new_config.save(CONFIG_FILE_PATH).await?;
    }

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
