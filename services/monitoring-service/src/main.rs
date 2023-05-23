use std::env;

use chrono::Utc;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, launch, post, routes};
use rocket_db_pools::Connection;
use rocket_db_pools::Database;
use service_lib::api_key::ApiKey;
use service_lib::api_key::ApiKeyVault;
use service_lib::database::MonitoringDb;
use service_lib::models::error_logs::ErrorLog;
use service_lib::models::os_infos;
use service_lib::models::system_informations;
use service_lib::profile_key::ProfileKey;

#[get("/")]
fn version() -> String {
    format!("{} v.{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

#[post("/error/<profile_id>")]
async fn error(
    _a_key: ApiKey<'_>,
    _p_key: ProfileKey<'_>,
    mut db: Connection<MonitoringDb>,
    profile_id: u32,
) -> Status {
    let error_log = ErrorLog::new(profile_id, "Test", Utc::now().naive_utc());

    if let Err(why) = error_log.insert(&mut *db).await {
        rocket::error!("Failed to insert error log: {why}");
        return Status::InternalServerError;
    };

    return Status::Ok;
}

#[post("/system-info/<profile_id>", data = "<info>")]
async fn system_info(
    _a_key: ApiKey<'_>,
    _p_key: ProfileKey<'_>,
    mut db: Connection<MonitoringDb>,
    profile_id: u32,
    info: Json<monitoring_core::models::SystemInformation>,
) -> Status {
    let hostname = match info.hostname.clone().into_string() {
        Ok(h) => h,
        Err(_) => {
            rocket::error!("Failed to convert hostname.");
            return Status::BadRequest;
        }
    };

    if let Ok(system_info_model) = system_informations::SystemInformation::new(
        profile_id as i32,
        &hostname,
        info.uptime.as_secs() as i64,
        info.boot_time,
        Utc::now().naive_utc(),
    )
    .insert(&mut *db)
    .await
    {
        if let Err(why) = os_infos::OsInfo::new(
            system_info_model.id_system_information,
            &info.os_info.os_type().to_string(),
            &info.os_info.version().to_string(),
            info.os_info.edition().and_then(|s| Some(String::from(s))),
            info.os_info.codename().and_then(|s| Some(String::from(s))),
            &info.os_info.bitness().to_string(),
            info.os_info
                .architecture()
                .and_then(|s| Some(String::from(s))),
        )
        .insert(&mut *db)
        .await
        {
            rocket::error!("Failed to insert os info: {why}.");
            return Status::InternalServerError;
        }

        rocket::info!("Inserted new system info for profile '{profile_id}'.");

        return Status::Ok;
    }

    Status::InternalServerError
}

#[launch]
fn rocket() -> _ {
    let api_key = env::var("MONITORING_API_KEY").expect("Expected api key in environment!");

    rocket::build()
        .manage(ApiKeyVault::new(&api_key))
        .attach(MonitoringDb::init())
        .mount("/", routes![error, system_info, version])
}
