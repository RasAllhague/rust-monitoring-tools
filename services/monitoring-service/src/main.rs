mod error_log;
mod service_info;
mod profile;

use std::env;

use profile::{get_profiles, save_profile};
use rocket::{get, launch, routes};
use rocket_db_pools::Database;
use service_info::{get_latest_entry, save_system_info};
use service_lib::api_key::ApiKeyVault;
use service_lib::database::MonitoringDb;
use service_lib::read_key::ReadKeyVault;

#[get("/")]
fn get_version() -> String {
    format!("{} v.{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

#[launch]
fn rocket() -> _ {
    let api_key = env::var("MONITORING_API_KEY").expect("Expected api key in environment!");
    let read_key = env::var("MONITORING_READ_KEY").expect("Expected api key in environment!");

    rocket::build()
        .manage(ApiKeyVault::new(&api_key))
        .manage(ReadKeyVault::new(&read_key))
        .attach(MonitoringDb::init())
        .mount(
            "/",
            routes![
                get_version,
                self::error_log::save_error,
                save_system_info,
                get_latest_entry,
                get_profiles,
                save_profile,
            ],
        )
}
