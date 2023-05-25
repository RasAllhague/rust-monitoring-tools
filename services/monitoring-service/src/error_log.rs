use chrono::Utc;
use rocket::http::Status;
use rocket::post;
use rocket_db_pools::Connection;
use service_lib::api_key::ApiKey;
use service_lib::database::MonitoringDb;
use service_lib::models::error_logs::ErrorLog;
use service_lib::profile_key::ProfileKey;

#[post("/error/<profile_id>")]
pub async fn save_error(
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
