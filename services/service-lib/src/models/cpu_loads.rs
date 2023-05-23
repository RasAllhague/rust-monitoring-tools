use sqlx::{pool::PoolConnection, Postgres};

pub struct CpuLoad {
    pub id_cpu_load: i32,
    pub user: f32,
    pub nice: f32,
    pub system: f32,
    pub interrupt: f32,
    pub idle: f32,
}

impl CpuLoad {
    pub fn new(user: f32, nice: f32, system: f32, interrupt: f32, idle: f32) -> Self {
        Self {
            id_cpu_load: 0,
            user,
            nice,
            system,
            interrupt,
            idle,
        }
    }

    pub async fn insert(self, db: &mut PoolConnection<Postgres>) -> sqlx::Result<Self> {
        let row: (i32,) = sqlx::query_as(
            "INSERT INTO cpu_loads 
            (user, nice, system, interrupt, idle) 
            VALUES 
            ($1, $2, $3, $4, $5) RETURNING id_cpu_load;")
            .bind(self.user)
            .bind(self.nice)
            .bind(self.system)
            .bind(self.interrupt)
            .bind(self.idle)
            .fetch_one(db)
            .await?;

        Ok(Self {
            id_cpu_load: row.0,
            user: self.user,
            nice: self.nice,
            system: self.system,
            interrupt: self.interrupt,
            idle: self.idle,
        })
    }
}