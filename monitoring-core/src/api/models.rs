use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InsertDeviceProfile {
    pub device_name: String,
    pub profile_key: String,
    pub create_user: i64,
}