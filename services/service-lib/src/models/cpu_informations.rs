use sqlx::{pool::PoolConnection, Postgres};

pub struct CpuInformation {
    pub id_cpu_information: i32,
    pub system_information_id: i32,
    pub temperature: f32,
    pub aggregate_load_id: i32,
}

impl CpuInformation {
    pub fn new(system_information_id: i32, temperature: f32, aggregate_load_id: i32) -> Self {
        Self {
            id_cpu_information: 0,
            system_information_id,
            temperature, 
            aggregate_load_id
        }
    }

    pub async fn insert(self, db: &mut PoolConnection<Postgres>) -> sqlx::Result<Self> {
        let row: (i32,) = sqlx::query_as(
            "INSERT INTO cpu_informations 
            (system_information_id, temperature, aggregate_load_id) 
            VALUES 
            ($1, $2, $3) RETURNING id_cpu_information;")
            .bind(self.system_information_id)
            .bind(self.temperature)
            .bind(self.aggregate_load_id)
            .fetch_one(db)
            .await?;

        Ok(Self {
            id_cpu_information: row.0,
            system_information_id: self.system_information_id,
            temperature: self.temperature,
            aggregate_load_id: self.aggregate_load_id,
        })
    }
}