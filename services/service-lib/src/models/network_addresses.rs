use sqlx::{pool::PoolConnection, Postgres};

pub struct NetworkAddress {
    pub id_network_address: i32,
    pub network_id: i32,
    pub address: String,
    pub netmask: String,
}

impl NetworkAddress {
    pub fn new(network_id: i32, address: &str, netmask: &str) -> Self {
        Self {
            id_network_address: 0,
            network_id,
            address: String::from(address),
            netmask: String::from(netmask),
        }
    }

    pub async fn insert(self, db: &mut PoolConnection<Postgres>) -> sqlx::Result<Self> {
        let row: (i32,) = sqlx::query_as(
            "INSERT INTO networks 
            (network_id, address, netmask) 
            VALUES 
            ($1, $2) RETURNING id_network_address;",
        )
        .bind(self.network_id)
        .bind(self.address.clone())
        .bind(self.netmask.clone())
        .fetch_one(db)
        .await?;

        Ok(Self {
            id_network_address: row.0,
            network_id: self.network_id,
            address: self.address,
            netmask: self.netmask,
        })
    }
}