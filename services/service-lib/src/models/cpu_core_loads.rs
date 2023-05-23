use sqlx::{pool::PoolConnection, Postgres};

pub struct CpuCoreLoad {
    pub id_cpu_core_load: i32,
    pub cpu_information_id: i32,
    pub cpu_load_id: i32,
}

impl CpuCoreLoad {
    pub fn new(cpu_information_id: i32, cpu_load_id: i32) -> Self {
        Self {
            id_cpu_core_load: 0,
            cpu_information_id,
            cpu_load_id,
        }
    }

    pub async fn insert(self, db: &mut PoolConnection<Postgres>) -> sqlx::Result<Self> {
        let row: (i32,) = sqlx::query_as(
            "INSERT INTO cpu_core_loads 
            (cpu_information_id, cpu_load_id) 
            VALUES 
            ($1, $2) RETURNING id_cpu_core_load;",
        )
        .bind(self.cpu_information_id)
        .bind(self.cpu_load_id)
        .fetch_one(db)
        .await?;

        Ok(Self {
            id_cpu_core_load: row.0,
            cpu_information_id: self.cpu_information_id,
            cpu_load_id: self.cpu_load_id,
        })
    }
}
