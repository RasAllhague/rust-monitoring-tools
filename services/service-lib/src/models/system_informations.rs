use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use sqlx::{pool::PoolConnection, Postgres, postgres::PgRow, Row};

#[derive(Serialize, Deserialize)]
pub struct SystemInformation {
    pub id_system_information: i32,
    pub device_profile_id: i32,
    pub hostname: String,
    pub uptime: i64,
    pub boot_time: NaiveDateTime,
    pub create_date: NaiveDateTime,
}

impl SystemInformation {
    pub fn new(
        device_profile_id: i32,
        hostname: &str,
        uptime: i64,
        boot_time: NaiveDateTime,
        create_date: NaiveDateTime,
    ) -> Self {
        Self {
            id_system_information: 0,
            device_profile_id,
            hostname: String::from(hostname),
            uptime,
            boot_time,
            create_date,
        }
    }

    pub async fn insert(self, db: &mut PoolConnection<Postgres>) -> sqlx::Result<Self> {
        let row: (i32,) = sqlx::query_as("INSERT INTO system_informations (device_profile_id, hostname, uptime, boot_time, create_date) VALUES ($1, $2, $3, $4, $5) RETURNING id_system_information;")
            .bind(self.device_profile_id)
            .bind(self.hostname.clone())
            .bind(self.uptime)
            .bind(self.boot_time)
            .bind(self.create_date)
            .fetch_one(db)
            .await?;

        Ok(Self {
            id_system_information: row.0,
            device_profile_id: self.device_profile_id,
            hostname: self.hostname,
            uptime: self.uptime,
            boot_time: self.boot_time,
            create_date: self.create_date,
        })
    }

    pub async fn get_lastest(db: &mut PoolConnection<Postgres>, profile_id: i32) -> sqlx::Result<Option<Self>> {
        let sql = 
            "SELECT * 
             FROM system_informations 
             WHERE profile_id = $1 
             ORDER BY CreateDate DESC
             LIMIT 1;";
    
        let row = sqlx::query(sql)
            .bind(profile_id)
            .fetch_optional(db)
            .await?;
 
         if let Some(row) = row {
             return Ok(Some(Self::from_row(&row)?));
         }
 
         return Ok(None);
    }

    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        Ok(Self {
            id_system_information: row.try_get(0)?,
            device_profile_id: row.try_get(1)?,
            hostname: row.try_get(2)?,
            uptime: row.try_get(3)?,
            boot_time: row.try_get(4)?,
            create_date: row.try_get(5)?,
        })
    }
}
