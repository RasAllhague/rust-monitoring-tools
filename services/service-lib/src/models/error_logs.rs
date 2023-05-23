use chrono::NaiveDateTime;
use sqlx::{pool::PoolConnection, Postgres};

#[derive(Debug, Clone)]
pub struct ErrorLog {
    pub id_error_log: i32,
    pub device_profile_id: i32,
    pub message: String,
    pub create_date: NaiveDateTime,
}

impl ErrorLog {
    pub fn new(device_profile_id: u32, message: &str, create_date: NaiveDateTime) -> Self {
        Self {
            id_error_log: 0,
            device_profile_id: device_profile_id as i32,
            message: String::from(message),
            create_date,
        }
    }

    pub async fn insert(self, db: &mut PoolConnection<Postgres>) -> sqlx::Result<Self> {
        let row: (i32,) = sqlx::query_as("INSERT INTO error_logs (device_profile_id, message, create_date) VALUES ($1, $2, $3) RETURNING id_error_log;")
            .bind(self.device_profile_id)
            .bind(self.message.clone())
            .bind(self.create_date)
            .fetch_one(db)
            .await?;

        Ok(Self {
            id_error_log: row.0,
            device_profile_id: self.device_profile_id,
            message: self.message,
            create_date: self.create_date,
        })
    }
}
