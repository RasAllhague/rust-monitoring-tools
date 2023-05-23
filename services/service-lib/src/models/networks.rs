use sqlx::{pool::PoolConnection, Postgres};

pub struct Network {
    pub id_network: i32,
    pub system_information_id: i32,
    pub name: String,
}

impl Network {
    pub fn new(system_information_id: i32, name: &str) -> Self {
        Self {
            id_network: 0,
            system_information_id,
            name: String::from(name),
        }
    }

    pub async fn insert(self, db: &mut PoolConnection<Postgres>) -> sqlx::Result<Self> {
        let row: (i32,) = sqlx::query_as(
            "INSERT INTO networks 
            (system_info_id, name) 
            VALUES 
            ($1, $2) RETURNING id_network;",
        )
        .bind(self.system_information_id)
        .bind(self.name.clone())
        .fetch_one(db)
        .await?;

        Ok(Self {
            id_network: row.0,
            system_information_id: self.system_information_id,
            name: self.name,
        })
    }
}
