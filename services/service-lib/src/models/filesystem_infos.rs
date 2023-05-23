use sqlx::{pool::PoolConnection, Postgres};

pub struct FilesystemInfo {
    pub id_filesystem_info: i32,
    pub system_information_id: i32,
    pub files: i32,
    pub files_total: i32,
    pub free: i64,
    pub avail: i64,
    pub total: i64,
    pub name_max: i32,
    pub fs_type: String,
    pub fs_mounted_from: String,
    pub fs_mounted_on: String,
}

impl FilesystemInfo {
    pub fn new(
        system_information_id: i32,
        files: i32,
        files_total: i32,
        free: i64,
        avail: i64,
        total: i64,
        name_max: i32,
        fs_type: &str,
        fs_mounted_from: &str,
        fs_mounted_on: &str,
    ) -> Self {
        Self {
            id_filesystem_info: 0,
            system_information_id,
            files,
            files_total,
            free,
            avail,
            total,
            name_max,
            fs_type: String::from(fs_type),
            fs_mounted_from: String::from(fs_mounted_from),
            fs_mounted_on: String::from(fs_mounted_on),
        }
    }

    pub async fn insert(self, db: &mut PoolConnection<Postgres>) -> sqlx::Result<Self> {
        let row: (i32,) = sqlx::query_as(
                "INSERT INTO filesystem_infos 
                (system_information_id, files, files_total, free, avail, total, name_max, fs_type, fs_mounted_from, fs_mounted_on) 
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) 
                RETURNING id_filesystem_info;")
            .bind(self.system_information_id)
            .bind(self.files)
            .bind(self.files_total)
            .bind(self.free)
            .bind(self.avail)
            .bind(self.total)
            .bind(self.name_max)
            .bind(self.fs_type.clone())
            .bind(self.fs_mounted_from.clone())
            .bind(self.fs_mounted_on.clone())
            .fetch_one(db)
            .await?;

        Ok(Self {
            id_filesystem_info: row.0,
            system_information_id: self.system_information_id,
            files: self.files,
            files_total: self.files_total,
            free: self.free,
            avail: self.avail,
            total: self.total,
            name_max: self.name_max,
            fs_type: self.fs_type,
            fs_mounted_from: self.fs_mounted_from,
            fs_mounted_on: self.fs_mounted_on,
        })
    }
}
