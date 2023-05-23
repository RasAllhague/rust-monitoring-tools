use rocket_db_pools::Database;

#[derive(Database)]
#[database("monitoring_db")]
pub struct MonitoringDb(sqlx::PgPool);