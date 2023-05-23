use chrono::NaiveDateTime;
use rocket_db_pools::Connection;

use crate::database::MonitoringDb;

pub struct DeviceProfile {
    pub id_device_profile: i32,
    pub device_name: String,
    pub profile_key: String,
    pub create_user: i64,
    pub create_date: NaiveDateTime,
    pub modify_user: Option<i64>,
    pub modify_date: Option<NaiveDateTime>,
}

impl DeviceProfile {
    pub fn new(id_device_profile: u32, device_name: &str, profile_key: &str, create_user: u64, create_date: NaiveDateTime) -> Self {
        Self {
            id_device_profile: id_device_profile as i32,
            device_name: String::from(device_name),
            profile_key: String::from(profile_key),
            create_user: create_user as i64,
            create_date,
            modify_date: None,
            modify_user: None,
        }
    }

    pub async fn get(mut db: Connection<MonitoringDb>, id: i32) -> sqlx::Result<Option<Self>> {
        let device_profile: Option<DeviceProfile> = sqlx::query_as!(
            DeviceProfile,
            "SELECT *
                FROM device_profiles
                WHERE id_device_profile = $1;",
            id,
        )
        .fetch_all(&mut *db)
        .await?
        .into_iter()
        .nth(0);

        Ok(device_profile)
    }
}