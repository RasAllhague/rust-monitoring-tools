use serde::{Deserialize, Serialize};

pub mod models;

#[derive(Serialize, Deserialize)]
pub struct ErrorLog {
    pub profile_id: u32,
    pub message: String,
}
