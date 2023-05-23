use sqlx::{pool::PoolConnection, Postgres};

pub struct SocketStatistic {
    pub id_socket_statistics: i32,
    pub system_information_id: i32,
    pub tcp_sockets_in_use: i32,
    pub tcp_sockets_orphaned: i32,
    pub udp_sockets_in_use: i32,
    pub tcp6_sockets_in_use: i32,
    pub udp6_sockets_in_use: i32,
}

impl SocketStatistic {
    pub fn new(
        system_information_id: i32,
        tcp_sockets_in_use: i32,
        tcp_sockets_orphaned: i32,
        udp_sockets_in_use: i32,
        tcp6_sockets_in_use: i32,
        udp6_sockets_in_use: i32,
    ) -> Self {
        Self {
            id_socket_statistics: 0,
            system_information_id,
            tcp_sockets_in_use,
            tcp6_sockets_in_use,
            tcp_sockets_orphaned,
            udp6_sockets_in_use,
            udp_sockets_in_use,
        }
    }

    pub async fn insert(self, db: &mut PoolConnection<Postgres>) -> sqlx::Result<Self> {
        let row: (i32,) = sqlx::query_as(
                "INSERT INTO socket_statistics 
                (system_information_id, tcp_sockets_in_use, tcp_sockets_orphaned, udp_sockets_in_use, tcp6_sockets_in_use, udp6_sockets_in_use) 
                VALUES ($1, $2, $3, $4, $5, $6) 
                RETURNING id_socket_statistics;")
            .bind(self.system_information_id)
            .bind(self.tcp_sockets_in_use)
            .bind(self.tcp_sockets_orphaned)
            .bind(self.udp_sockets_in_use)
            .bind(self.tcp6_sockets_in_use)
            .bind(self.udp6_sockets_in_use)
            .fetch_one(db)
            .await?;

        Ok(Self {
            id_socket_statistics: row.0,
            system_information_id: self.system_information_id,
            tcp_sockets_in_use: self.tcp_sockets_in_use,
            tcp_sockets_orphaned: self.tcp_sockets_orphaned,
            udp_sockets_in_use: self.udp_sockets_in_use,
            tcp6_sockets_in_use: self.tcp6_sockets_in_use,
            udp6_sockets_in_use: self.udp6_sockets_in_use,
        })
    }
}
