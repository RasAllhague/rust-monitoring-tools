use sqlx::{pool::PoolConnection, Postgres};

pub struct LoadAverage {
    pub id_load_average: i32,
    pub system_information_id: i32,
    pub one: i32,
    pub five: i32,
    pub fifteen: i32,
}

impl LoadAverage {
    pub fn new(system_information_id: i32, one: i32, five: i32, fifteen: i32) -> Self {
        Self {
            id_load_average: 0,
            system_information_id,
            one,
            five,
            fifteen,
        }
    }

    pub async fn insert(self, db: &mut PoolConnection<Postgres>) -> sqlx::Result<Self> {
        let row: (i32,) = sqlx::query_as(
            "INSERT INTO cpu_core_loads 
            (system_information_id, one, five, fifteen) 
            VALUES 
            ($1, $2, $3, $4) RETURNING id_load_average;",
        )
        .bind(self.system_information_id)
        .bind(self.one)
        .bind(self.five)
        .bind(self.fifteen)
        .fetch_one(db)
        .await?;

        Ok(Self {
            id_load_average: row.0,
            system_information_id: self.system_information_id,
            one: self.one,
            five: self.five,
            fifteen: self.fifteen,
        })
    }
}
