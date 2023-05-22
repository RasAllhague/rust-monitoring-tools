mod config;

use monitoring_core::models::{SystemInformation};
use systemstat::{Platform, System};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let system = System::new();
    let hostname = hostname::get()?;
    let os_info = os_info::get();

    let system_info = SystemInformation::collect(system, hostname, os_info)?;
    
    let resp = reqwest::Client::new()
        .post("http://127.0.0.1:8000/system-info")
        .json(&system_info)
        .send()
        .await?;

    println!("Statuscode: {}", resp.status());

    Ok(())
}