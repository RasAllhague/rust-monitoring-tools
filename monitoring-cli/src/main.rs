mod config;

use config::Opt;
use monitoring_core::{models::SystemInformation, ErrorLog};
use structopt::StructOpt;
use systemstat::{Platform, System, Duration};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    match opt {
        Opt::Error { message, api_key, id, profile_key } => post_error_log_message(&profile_key, &message, &api_key, id).await?,
        Opt::Single { api_key, id, profile_key } => post_system_info(&profile_key, &api_key, id).await?,
        Opt::Service { api_key, sleep_seconds, id, profile_key } => run_service(&profile_key, &api_key, sleep_seconds, id).await,
    }
    
    Ok(())
}

async fn post_system_info(profile_key: &str, api_key: &str, profile_id: u32) -> anyhow::Result<()> {
    let system = System::new();
    let hostname = hostname::get()?;
    let os_info = os_info::get();

    let system_info = SystemInformation::collect(system, hostname, os_info)?;

    let resp = reqwest::Client::new()
        .post(format!("http://127.0.0.1:8000/system-info/{}", profile_id))
        .header("x-api-key", api_key)
        .header("x-profile-key", profile_key)
        .json(&system_info)
        .send()
        .await?;

    println!("Statuscode: {}", resp.status());

    Ok(())
}

async fn post_error_log_message(profile_key: &str, message: &str, api_key: &str, profile_id: u32) -> anyhow::Result<()> {
    let resp = reqwest::Client::new()
        .post(format!("http://127.0.0.1:8000/error/{}", profile_id))
        .header("x-api-key", api_key)
        .header("x-profile-key", profile_key)
        .json(&ErrorLog {
            message: String::from(message),
            profile_id,
        })
        .send()
        .await?;

    println!("Send error log: Statuscode: {}", resp.status());

    Ok(())
}

async fn run_service(profile_key: &str, api_key: &str, sleep_seconds: u64, profile_id: u32) {
    loop {
        if let Err(why) = post_system_info(profile_key, api_key, profile_id).await {
            let message = format!("Service failed to collect data: {why}");
            println!("{message}");
            
            if let Err(inner_why) = post_error_log_message(profile_key, &message, api_key, profile_id).await {
                let message = format!("Service failed to send error log: {inner_why}");
                println!("{message}");
            }
        }

        sleep(Duration::from_secs(sleep_seconds)).await;
    }
}