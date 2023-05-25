use chrono::Utc;
use monitoring_core::api::models::InsertDeviceProfile;
use rocket::{error, get, http::Status, post, serde::json::Json};
use rocket_db_pools::Connection;
use service_lib::{
    api_key::ApiKey, database::MonitoringDb, models::device_profiles::DeviceProfile,
    read_key::ReadKey,
};

#[get("/profiles")]
pub async fn get_profiles(
    _a_key: ApiKey<'_>,
    _r_key: ReadKey<'_>,
    mut db: Connection<MonitoringDb>,
) -> Result<Json<Vec<DeviceProfile>>, Status> {
    match DeviceProfile::get_all(&mut *db).await {
        Ok(profiles) => Ok(Json(profiles)),
        Err(why) => {
            error!("Failed to get devices profiles from db: {why}");

            Err(Status::InternalServerError)
        }
    }
}

#[post("/profiles", data = "<profile>")]
pub async fn save_profile(_a_key: ApiKey<'_>, mut db: Connection<MonitoringDb>, profile: Json<InsertDeviceProfile>) -> Status {
    let device_profile = DeviceProfile::new(
        &profile.device_name,
        &profile.profile_key,
        profile.create_user,
        Utc::now().naive_utc(),
    );

    if let Err(why) = device_profile.insert(&mut *db).await {
        error!("Failed to create new profile: {why}");

        return Status::InternalServerError;
    }

    Status::Ok
}
