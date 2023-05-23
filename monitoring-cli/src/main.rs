mod config;

use config::Opt;
use monitoring_core::models::SystemInformation;
use structopt::StructOpt;
use systemstat::{Platform, System, Duration};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    match opt {
        Opt::Single { api_key, profile_id } => run_single(&api_key, profile_id).await?,
        Opt::Service { api_key, sleep_seconds, profile_id } => run_service(&api_key, sleep_seconds, profile_id).await,
    }
    
    Ok(())
}

async fn run_single(api_key: &str, profile_id: u32) -> anyhow::Result<()> {
    let system = System::new();
    let hostname = hostname::get()?;
    let os_info = os_info::get();

    let system_info = SystemInformation::collect(system, hostname, os_info)?;

    let resp = reqwest::Client::new()
        .post(format!("http://127.0.0.1:8000/system-info/{}", profile_id))
        .header("x-api-key", api_key)
        .json(&system_info)
        .send()
        .await?;

    println!("Statuscode: {}", resp.status());

    Ok(())
}

async fn run_service(api_key: &str, sleep_seconds: u64, profile_id: u32) {
    loop {
        if let Err(why) = run_single(api_key, profile_id).await {
            println!("Service failed to collect data: {}", why);
        }

        sleep(Duration::from_secs(sleep_seconds)).await;
    }
}