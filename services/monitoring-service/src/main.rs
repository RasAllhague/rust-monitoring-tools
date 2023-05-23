use std::env;

use chrono::Utc;
use monitoring_core::models::SystemInformation;
use rocket::http::Status;
use rocket::serde::json::serde_json;
use rocket::serde::json::Json;
use rocket::tokio::fs::File;
use rocket::tokio::io::AsyncWriteExt;
use rocket::{get, launch, post, routes};
use rocket_db_pools::Connection;
use rocket_db_pools::Database;
use service_lib::api_key::ApiKey;
use service_lib::api_key::ApiKeyVault;
use service_lib::database::MonitoringDb;
use service_lib::models::device_profiles::DeviceProfile;
use service_lib::models::error_logs::ErrorLog;
use service_lib::profile_key::ProfileKey;

#[get("/")]
fn version() -> String {
    format!("{} v.{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

#[post("/error/<profile_id>")]
async fn error(_a_key: ApiKey<'_>, _p_key: ProfileKey<'_>, mut db: Connection<MonitoringDb>, profile_id: u32) -> Status {
    if let Ok(device_profile) = DeviceProfile::get(&mut *db, profile_id as i32).await {
        if let None = device_profile {
            return Status::BadRequest;
        }

        let error_log = ErrorLog::new(profile_id, "Test", Utc::now().naive_utc());
            
        if let Err(why) = error_log.insert(&mut *db).await {
            rocket::error!("Failed to insert error log: {why}");
            return Status::InternalServerError;
        };

        return Status::Ok;
    }   

    Status::BadRequest
}

#[post("/system-info/<profile_id>", data = "<info>")]
async fn system_info(_a_key: ApiKey<'_>, _p_key: ProfileKey<'_>, mut db: Connection<MonitoringDb>, profile_id: u32, info: Json<SystemInformation>) -> std::io::Result<()> {
    let mut file = File::create("test.json").await?;
    file.write_all(serde_json::to_string(&info.0).unwrap().as_bytes())
        .await?;

    Ok(())
}

#[launch]
fn rocket() -> _ {
    let api_key = env::var("MONITORING_API_KEY").expect("Expected api key in environment!");

    rocket::build()
        .manage(ApiKeyVault::new(&api_key))
        .attach(MonitoringDb::init())
        .mount("/", routes![error, system_info, version])
}
