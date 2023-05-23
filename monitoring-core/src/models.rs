use std::{
    collections::HashMap,
    ffi::OsString,
    net::{Ipv4Addr, Ipv6Addr},
    thread::sleep,
    time::Duration,
};

use chrono::{NaiveDate, NaiveDateTime};
use os_info::Info;
use serde::{Deserialize, Serialize};
use systemstat::{CPULoad, Platform, System};

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemInformation {
    pub hostname: OsString,
    pub os_info: Info,
    pub cpu: CpuInformation,
    pub load_avg: LoadAverage,
    pub memory: Memory,
    pub swap: Swap,
    pub battery_life: Option<BatteryLife>,
    pub mounts: Vec<Filesystem>,
    pub networks: HashMap<String, Network>,
    pub net_stats: HashMap<String, NetworkStatistics>,
    pub socket_stats: SocketStatistics,
    pub uptime: Duration,
    pub boot_time: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CpuInformation {
    pub temperature: f32,
    pub loads: Vec<CpuLoad>,
    pub aggregate_load: CpuLoad,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CpuLoad {
    pub user: f32,
    pub nice: f32,
    pub system: f32,
    pub interrupt: f32,
    pub idle: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoadAverage {
    pub one: f32,
    pub five: f32,
    pub fifteen: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Memory {
    pub total: u64,
    pub free: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Swap {
    pub total: u64,
    pub free: u64,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct BatteryLife {
    pub remaining_capacity: f32,
    pub remaining_time: Duration,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum IpAddress {
    Empty,
    Unsupported,
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkAddress {
    pub addr: IpAddress,
    pub netmask: IpAddress,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Network {
    pub name: String,
    pub addrs: Vec<NetworkAddress>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkStatistics {
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_packets: u64,
    pub tx_packets: u64,
    pub rx_errors: u64,
    pub tx_errors: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SocketStatistics {
    pub tcp_sockets_in_use: usize,
    pub tcp_sockets_orphaned: usize,
    pub udp_sockets_in_use: usize,
    pub tcp6_sockets_in_use: usize,
    pub udp6_sockets_in_use: usize,
}

impl SystemInformation {
    pub fn collect(system: System, hostname: OsString, os_info: Info) -> anyhow::Result<Self> {
        let mut mounts = Vec::new();

        for fs in system.mounts()? {
            mounts.push(Filesystem::from(fs));
        }

        let mut networks = HashMap::new();

        for (key, value) in system.networks()? {
            networks.insert(key, Network::from(value));
        }

        let mut net_stats = HashMap::new();

        for (network, _) in system.networks()? {
            net_stats.insert(network.clone(), NetworkStatistics::from(system.network_stats(&network)?));
        }

        let boot_time = {
            let boot_time = system.boot_time()?;
            NaiveDate::from_ymd_opt(
                boot_time.year(),
                boot_time.month() as u32,
                boot_time.day() as u32,
            )
            .unwrap()
            .and_hms_opt(
                boot_time.hour() as u32,
                boot_time.minute() as u32,
                boot_time.second() as u32,
            )
            .unwrap()
        };

        let battery_life = match system.battery_life() {
            Ok(b) => Some(BatteryLife::from(b)),
            Err(_) => None,
        };

        Ok(Self {
            hostname,
            os_info,
            cpu: CpuInformation::collect(&system)?,
            load_avg: LoadAverage::from(system.load_average()?),
            memory: Memory::from(system.memory()?),
            swap: Swap::from(system.swap()?),
            battery_life: battery_life,
            mounts,
            networks,
            net_stats,
            socket_stats: SocketStatistics::from(system.socket_stats()?),
            uptime: system.uptime()?,
            boot_time: boot_time,
        })
    }
}

impl CpuInformation {
    pub fn collect(system: &System) -> anyhow::Result<Self> {
        let temp = system.cpu_temp()?;
        let (cpu_loads, load_aggregate) = {
            let load = system.cpu_load()?;
            let aggregate = system.cpu_load_aggregate()?;

            sleep(Duration::from_secs(1));

            (load.done()?, aggregate.done()?)
        };

        let mut loads = Vec::new();

        for load in cpu_loads {
            loads.push(CpuLoad::from(load));
        }

        Ok(Self {
            temperature: temp,
            loads: loads,
            aggregate_load: CpuLoad::from(load_aggregate),
        })
    }
}

impl From<CPULoad> for CpuLoad {
    fn from(value: CPULoad) -> Self {
        Self {
            user: value.user,
            nice: value.nice,
            system: value.system,
            interrupt: value.interrupt,
            idle: value.idle,
        }
    }
}

impl From<systemstat::LoadAverage> for LoadAverage {
    fn from(value: systemstat::LoadAverage) -> Self {
        Self {
            one: value.one,
            five: value.five,
            fifteen: value.fifteen,
        }
    }
}

impl From<systemstat::Memory> for Memory {
    fn from(value: systemstat::Memory) -> Self {
        Self {
            total: value.total.0,
            free: value.free.0,
        }
    }
}

impl From<systemstat::Swap> for Swap {
    fn from(value: systemstat::Swap) -> Self {
        Self {
            total: value.total.0,
            free: value.free.0,
        }
    }
}

impl From<systemstat::BatteryLife> for BatteryLife {
    fn from(value: systemstat::BatteryLife) -> Self {
        Self {
            remaining_capacity: value.remaining_capacity,
            remaining_time: value.remaining_time,
        }
    }
}

impl From<systemstat::Filesystem> for Filesystem {
    fn from(value: systemstat::Filesystem) -> Self {
        Self {
            files: value.files,
            files_total: value.files_total,
            files_avail: value.files_avail,
            free: value.free.0,
            avail: value.avail.0,
            total: value.total.0,
            name_max: value.name_max,
            fs_type: value.fs_type,
            fs_mounted_from: value.fs_mounted_from,
            fs_mounted_on: value.fs_mounted_on,
        }
    }
}

impl From<systemstat::Network> for Network {
    fn from(value: systemstat::Network) -> Self {
        let mut addrs = Vec::new();

        for addr in value.addrs {
            addrs.push(NetworkAddress::from(addr));
        }

        Self {
            name: value.name,
            addrs,
        }
    }
}

impl From<systemstat::NetworkAddrs> for NetworkAddress {
    fn from(value: systemstat::NetworkAddrs) -> Self {
        Self {
            addr: IpAddress::from(value.addr),
            netmask: IpAddress::from(value.netmask),
        }
    }
}

impl From<systemstat::IpAddr> for IpAddress {
    fn from(value: systemstat::IpAddr) -> Self {
        match value {
            systemstat::IpAddr::Empty => IpAddress::Empty,
            systemstat::IpAddr::Unsupported => IpAddress::Unsupported,
            systemstat::IpAddr::V4(v4) => IpAddress::V4(v4),
            systemstat::IpAddr::V6(v6) => IpAddress::V6(v6),
        }
    }
}

impl From<systemstat::NetworkStats> for NetworkStatistics {
    fn from(value: systemstat::NetworkStats) -> Self {
        Self {
            rx_bytes: value.rx_bytes.0,
            tx_bytes: value.tx_bytes.0,
            rx_packets: value.rx_packets,
            tx_packets: value.tx_packets,
            rx_errors: value.rx_errors,
            tx_errors: value.tx_errors,
        }
    }
}

impl From<systemstat::SocketStats> for SocketStatistics {
    fn from(value: systemstat::SocketStats) -> Self {
        Self {
            tcp6_sockets_in_use: value.tcp6_sockets_in_use,
            tcp_sockets_in_use: value.tcp_sockets_in_use,
            udp6_sockets_in_use: value.udp6_sockets_in_use,
            udp_sockets_in_use: value.udp_sockets_in_use,
            tcp_sockets_orphaned: value.tcp_sockets_orphaned,
        }
    }
}
