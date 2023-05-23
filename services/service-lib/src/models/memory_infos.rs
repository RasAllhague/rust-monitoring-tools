use sqlx::{pool::PoolConnection, Postgres};

pub struct MemoryInfo {
    pub id_memory_info: i32,
    pub system_information_id: i32,
    pub free: i64,
    pub total: i64,
}

impl MemoryInfo {
    pub fn new(system_information_id: i32, free: i64, total: i64) -> Self {
        Self {
            id_memory_info: 0,
            system_information_id,
            free,
            total,
        }
    }

    pub async fn insert(self, db: &mut PoolConnection<Postgres>) -> sqlx::Result<Self> {
        let row: (i32,) = sqlx::query_as(
            "INSERT INTO memory_infos 
            (system_information_id, free, total) 
            VALUES 
            ($1, $2, $3) RETURNING id_memory_info;",
        )
        .bind(self.system_information_id)
        .bind(self.free)
        .bind(self.total)
        .fetch_one(db)
        .await?;

        Ok(Self {
            id_memory_info: row.0,
            system_information_id: self.system_information_id,
            free: self.free,
            total: self.total,
        })
    }
}
