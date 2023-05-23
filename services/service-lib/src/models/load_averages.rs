use sqlx::{pool::PoolConnection, Postgres};

pub struct LoadAverage {
    pub id_load_average: i32,
    pub system_information_id: i32,
    pub one: f32,
    pub five: f32,
    pub fifteen: f32,
}

impl LoadAverage {
    pub fn new(system_information_id: i32, one: f32, five: f32, fifteen: f32) -> Self {
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
            "INSERT INTO load_averages 
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
