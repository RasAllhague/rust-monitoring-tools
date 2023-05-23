use sqlx::{pool::PoolConnection, Postgres};

pub struct NetworkStatistic {
    pub id_network_statistics: i32,
    pub system_information_id: i32,
    pub rx_bytes: i64,
    pub tx_bytes: i64,
    pub rx_packages: i64,
    pub tx_packages: i64,
    pub rx_errors: i64,
    pub tx_errors: i64,
}

impl NetworkStatistic {
    pub fn new(system_information_id: i32, rx_bytes: i64, tx_bytes: i64, rx_packages: i64, tx_packages: i64, rx_errors: i64, tx_errors: i64) -> Self {
        Self {
            id_network_statistics: 0,
            system_information_id,
            rx_bytes,
            tx_bytes,
            rx_packages,
            tx_packages,
            rx_errors,
            tx_errors
        }
    }

    pub async fn insert(self, db: &mut PoolConnection<Postgres>) -> sqlx::Result<Self> {
        let row: (i32,) = sqlx::query_as(
                "INSERT INTO networks_statistics 
                (system_information_id, rx_bytes, tx_bytes, rx_packages, tx_packages, rx_errors, tx_errors) 
                VALUES ($1, $2, $3, $4, $5, $6, $7) 
                RETURNING id_network_statistics;")
            .bind(self.system_information_id)
            .bind(self.rx_bytes)
            .bind(self.tx_bytes)
            .bind(self.rx_packages)
            .bind(self.tx_packages)
            .bind(self.rx_errors)
            .bind(self.tx_errors)
            .fetch_one(db)
            .await?;

        Ok(Self {
            id_network_statistics: row.0,
            system_information_id: self.system_information_id,
            rx_bytes: self.rx_bytes,
            tx_bytes: self.tx_bytes,
            rx_packages: self.rx_packages,
            tx_packages: self.tx_packages,
            rx_errors: self.rx_errors,
            tx_errors: self.tx_errors,
        })
    }
}