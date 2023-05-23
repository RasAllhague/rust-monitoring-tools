use sqlx::{pool::PoolConnection, Postgres};

pub struct OsInfo {
    pub id_os_info: i32,
    pub system_information_id: i32,
    pub os_type: String,
    pub version: String,
    pub edition: Option<String>,
    pub codename: Option<String>,
    pub bitness: String,
    pub architecture: Option<String>,
}

impl OsInfo {
    pub fn new(
        system_info_id: i32,
        os_type: &str,
        version: &str,
        edition: Option<String>,
        codename: Option<String>,
        bitness: &str,
        architecture: Option<String>,
    ) -> Self {
        Self {
            id_os_info: 0,
            system_information_id: system_info_id,
            os_type: String::from(os_type),
            version: String::from(version),
            edition,
            codename,
            bitness: String::from(bitness),
            architecture,
        }
    }

    pub async fn insert(self, db: &mut PoolConnection<Postgres>) -> sqlx::Result<Self> {
        let row: (i32,) = sqlx::query_as(
            "INSERT INTO os_infos 
            (system_information_id, os_type, version, edition, codename, bitness, architecture) 
            VALUES 
            ($1, $2, $3, $4, $5, $6, $7) RETURNING id_os_info;",
        )
        .bind(self.system_information_id)
        .bind(self.os_type.clone())
        .bind(self.version.clone())
        .bind(self.edition.clone())
        .bind(self.codename.clone())
        .bind(self.bitness.clone())
        .bind(self.architecture.clone())
        .fetch_one(db)
        .await?;

        Ok(Self {
            id_os_info: row.0,
            system_information_id: self.system_information_id,
            os_type: self.os_type,
            version: self.version,
            edition: self.edition,
            codename: self.codename,
            bitness: self.bitness,
            architecture: self.architecture,
        })
    }
}
