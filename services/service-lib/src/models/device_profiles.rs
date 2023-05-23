use chrono::NaiveDateTime;
use sqlx::{pool::PoolConnection, Postgres, Row};

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
    pub fn new(
        id_device_profile: u32,
        device_name: &str,
        profile_key: &str,
        create_user: u64,
        create_date: NaiveDateTime,
    ) -> Self {
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

    pub async fn get(db: &mut PoolConnection<Postgres>, id: i32) -> sqlx::Result<Option<Self>> {
        let row = sqlx::query("SELECT * FROM device_profiles WHERE id_device_profile = $1;")
            .bind(id)
            .fetch_optional(db)
            .await?;

        if let Some(profile_row) = row {
            return Ok(Some(DeviceProfile {
                id_device_profile: profile_row.try_get(0)?,
                device_name: profile_row.try_get(1)?,
                profile_key: profile_row.try_get(2)?,
                create_user: profile_row.try_get(3)?,
                create_date: profile_row.try_get(4)?,
                modify_user: profile_row.try_get(5)?,
                modify_date: profile_row.try_get(6)?,
            }));
        }

        return Ok(None);
    }
}
