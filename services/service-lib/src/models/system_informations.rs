use chrono::NaiveDateTime;

pub struct SystemInformation {
    pub id_system_information: i32,
    pub device_profile_id: i32,
    pub hostname: String,
    pub uptime: i32,
    pub boot_time: NaiveDateTime,
    pub create_date: NaiveDateTime,
}