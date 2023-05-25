use serde::{Deserialize, Serialize};

pub mod models;
pub mod options;
pub mod client;
pub mod api;

#[derive(Serialize, Deserialize)]
pub struct ErrorLog {
    pub profile_id: u32,
    pub message: String,
}
