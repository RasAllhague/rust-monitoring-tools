mod error_log;
mod service_info;

use std::env;

use rocket::{get, launch, routes};
use rocket_db_pools::Database;
use service_info::{get_latest_entry, save_system_info};
use service_lib::api_key::ApiKeyVault;
use service_lib::database::MonitoringDb;

#[get("/")]
fn get_version() -> String {
    format!("{} v.{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

#[launch]
fn rocket() -> _ {
    let api_key = env::var("MONITORING_API_KEY").expect("Expected api key in environment!");

    rocket::build()
        .manage(ApiKeyVault::new(&api_key))
        .attach(MonitoringDb::init())
        .mount(
            "/",
            routes![
                get_version,
                self::error_log::save_error,
                save_system_info,
                get_latest_entry
            ],
        )
}
