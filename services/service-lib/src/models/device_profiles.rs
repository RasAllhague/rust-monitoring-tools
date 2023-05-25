use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{pool::PoolConnection, postgres::PgRow, Postgres, Row};

#[derive(Serialize, Deserialize)]
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
        device_name: &str,
        profile_key: &str,
        create_user: i64,
        create_date: NaiveDateTime,
    ) -> Self {
        Self {
            id_device_profile: 0,
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
            return Ok(Some(Self::from_row(&profile_row)?));
        }

        return Ok(None);
    }

    pub async fn get_all(db: &mut PoolConnection<Postgres>) -> sqlx::Result<Vec<DeviceProfile>> {
        sqlx::query("SELECT * FROM device_profiles;")
            .fetch_all(db)
            .await?
            .iter()
            .map(|x| Self::from_row(x))
            .collect::<sqlx::Result<Vec<DeviceProfile>>>()
    }

    pub async fn insert(self, db: &mut PoolConnection<Postgres>) -> sqlx::Result<Self> {
        let row: (i32,) = sqlx::query_as(
            "INSERT INTO device_profiles 
            (id_device_profile, device_name, profile_key, create_user, create_date) 
            VALUES 
            ($1, $2, $3, $4, $5) RETURNING id_device_profile;",
        )
        .bind(self.id_device_profile)
        .bind(self.device_name.clone())
        .bind(self.profile_key.clone())
        .bind(self.create_user)
        .bind(self.create_date)
        .fetch_one(db)
        .await?;

        Ok(Self {
            id_device_profile: row.0,
            device_name: self.device_name.clone(),
            profile_key: self.profile_key.clone(),
            create_user: self.create_user,
            create_date: self.create_date,
            modify_date: None,
            modify_user: None,
        })
    }

    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        Ok(Self {
            id_device_profile: row.try_get(0)?,
            device_name: row.try_get(1)?,
            profile_key: row.try_get(2)?,
            create_user: row.try_get(3)?,
            create_date: row.try_get(4)?,
            modify_user: row.try_get(5)?,
            modify_date: row.try_get(6)?,
        })
    }
}
