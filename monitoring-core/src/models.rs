use std::{time::Duration, net::{Ipv4Addr, Ipv6Addr}, collections::HashMap};

use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SystemInformation {
    pub cpu: CpuInformation,
    pub load_avg: LoadAverage,
    pub memory: Memory,
    pub swap: Swap,
    pub battery_life: Option<BatteryLife>,
    pub mounts: Vec<Filesystem>,
    pub networks: HashMap<String, Network>,
    pub net_stats: NetworkStatistics,
    pub socket_stats: SocketStatistics,
    pub uptime: Duration,
    pub boot_time: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct CpuInformation {
    pub temperature: f32,
    pub loads: Vec<CpuLoad>,
    pub aggregate_load: CpuLoad,
}

#[derive(Serialize, Deserialize)]
pub struct CpuLoad {
    pub user: f32,
    pub nice: f32,
    pub system: f32,
    pub interrupt: f32,
    pub idle: f32,
}

#[derive(Serialize, Deserialize)]
pub struct LoadAverage {
    pub one: f32,
    pub five: f32,
    pub fifteen: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Memory {
    total: u64,
    free: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Swap {
    total: u64,
    free: u64,
}

#[derive(Serialize, Deserialize)]
pub struct BatteryLife {
    pub remaining_capacity: f32,
    pub remaining_time: Duration,
}

#[derive(Serialize, Deserialize)]
pub struct Filesystem {
    pub files: usize,
    pub files_total: usize,
    pub files_avail: usize,
    pub free: u64,
    pub avail: u64,
    pub total: u64,
    pub name_max: usize,
    pub fs_type: String,
    pub fs_mounted_from: String,
    pub fs_mounted_on: String,
}

#[derive(Serialize, Deserialize)]
pub enum IpAddress {
    Empty,
    Unsupported,
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

#[derive(Serialize, Deserialize)]
pub struct NetworkAddress {
    pub addr: IpAddress,
    pub netmask: IpAddress,
}

#[derive(Serialize, Deserialize)]
pub struct Network {
    pub name: String,
    pub addrs: Vec<NetworkAddress>,
}

#[derive(Serialize, Deserialize)]
pub struct NetworkStatistics {
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_packets: u64,
    pub tx_packets: u64,
    pub rx_errors: u64,
    pub tx_errors: u64,
}

#[derive(Serialize, Deserialize)]
pub struct SocketStatistics {
    pub tcp_sockets_in_use: usize,
    pub tcp_sockets_orphaned: usize,
    pub udp_sockets_in_use: usize,
    pub tcp6_sockets_in_use: usize,
    pub udp6_sockets_in_use: usize,
}