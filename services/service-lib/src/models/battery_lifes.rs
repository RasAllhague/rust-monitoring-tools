use sqlx::{pool::PoolConnection, Postgres};

pub struct BatteryLife {
    pub id_battery_life: i32,
    pub system_information_id: i32,
    pub remaining_capacity: f32,
    pub remaining_time: i64,
}

impl BatteryLife {
    pub fn new(system_information_id: i32, remaining_capacity: f32, remaining_time: i64) -> Self {
        Self {
            id_battery_life: 0,
            system_information_id,
            remaining_capacity,
            remaining_time,
        }
    }

    pub async fn insert(self, db: &mut PoolConnection<Postgres>) -> sqlx::Result<Self> {
        let row: (i32,) = sqlx::query_as(
            "INSERT INTO memory_infos 
            (system_information_id, remaining_capacity, remaining_time) 
            VALUES 
            ($1, $2, $3) RETURNING id_battery_life;",
        )
        .bind(self.system_information_id)
        .bind(self.remaining_capacity)
        .bind(self.remaining_time)
        .fetch_one(db)
        .await?;

        Ok(Self {
            id_battery_life: row.0,
            system_information_id: self.system_information_id,
            remaining_capacity: self.remaining_capacity,
            remaining_time: self.remaining_time,
        })
    }
}
