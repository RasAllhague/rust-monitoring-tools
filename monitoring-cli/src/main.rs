mod config;

use monitoring_core::models::{SystemInformation};
use systemstat::{Platform, System};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let system = System::new();
    let hostname = hostname::get()?;
    let os_info = os_info::get();

    let system_info = SystemInformation::collect(system, hostname, os_info)?;
    
    

    Ok(())
}