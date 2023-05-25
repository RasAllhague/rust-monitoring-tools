use chrono::Utc;
use monitoring_core::models::IpAddress;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, post};
use rocket_db_pools::Connection;
use service_lib::api_key::ApiKey;
use service_lib::database::MonitoringDb;
use service_lib::models::battery_lifes;
use service_lib::models::cpu_core_loads;
use service_lib::models::cpu_informations;
use service_lib::models::cpu_loads;
use service_lib::models::filesystem_infos;
use service_lib::models::load_averages;
use service_lib::models::memory_infos;
use service_lib::models::network_addresses;
use service_lib::models::network_statistics;
use service_lib::models::networks;
use service_lib::models::os_infos;
use service_lib::models::socket_statistics;
use service_lib::models::swap_infos;
use service_lib::models::system_informations;
use service_lib::profile_key::ProfileKey;
use sqlx::pool::PoolConnection;
use sqlx::Postgres;

#[get("/system-info/<profile_id>")]
pub async fn get_latest_entry(
    _a_key: ApiKey<'_>,
    _p_key: ProfileKey<'_>,
    mut db: Connection<MonitoringDb>,
    profile_id: u32,
) -> Status {
    Status::NotImplemented
}

#[post("/system-info/<profile_id>", data = "<info>")]
pub async fn save_system_info(
    _a_key: ApiKey<'_>,
    _p_key: ProfileKey<'_>,
    mut db: Connection<MonitoringDb>,
    profile_id: u32,
    info: Json<monitoring_core::models::SystemInformation>,
) -> Status {
    let hostname = match info.hostname.clone().into_string() {
        Ok(h) => h,
        Err(_) => {
            rocket::error!("Failed to convert hostname.");
            return Status::BadRequest;
        }
    };

    if let Ok(system_info_model) = system_informations::SystemInformation::new(
        profile_id as i32,
        &hostname,
        info.uptime.as_secs() as i64,
        info.boot_time,
        Utc::now().naive_utc(),
    )
    .insert(&mut *db)
    .await
    {
        if let Some(os) = &info.os_info {
            if let Err(why) = os_infos::OsInfo::new(
                system_info_model.id_system_information,
                &os.os_type().to_string(),
                &os.version().to_string(),
                os.edition().and_then(|s| Some(String::from(s))),
                os.codename().and_then(|s| Some(String::from(s))),
                &os.bitness().to_string(),
                os.architecture().and_then(|s| Some(String::from(s))),
            )
            .insert(&mut *db)
            .await
            {
                rocket::error!("Failed to insert os info: {why}.");
            }
        }

        if let Err(why) =
            insert_cpu_data(system_info_model.id_system_information, &info, &mut *db).await
        {
            rocket::error!("Failed to insert cpu infos: {why}.");
        }

        if let Some(l_avg) = &info.load_avg {
            if let Err(why) = load_averages::LoadAverage::new(
                system_info_model.id_system_information,
                l_avg.one,
                l_avg.five,
                l_avg.fifteen,
            )
            .insert(&mut *db)
            .await
            {
                rocket::error!("Failed to insert load averages: {why}.");
            }
        }

        if let Some(mem) = &info.memory {
            if let Err(why) = memory_infos::MemoryInfo::new(
                system_info_model.id_system_information,
                mem.free as i64,
                mem.total as i64,
            )
            .insert(&mut *db)
            .await
            {
                rocket::error!("Failed to insert memory info: {why}.");
            }
        }

        if let Some(swap) = &info.swap {
            if let Err(why) = swap_infos::SwapInfo::new(
                system_info_model.id_system_information,
                swap.free as i64,
                swap.total as i64,
            )
            .insert(&mut *db)
            .await
            {
                rocket::error!("Failed to insert swap info: {why}.");
            }
        }

        if let Some(battery_life) = info.battery_life {
            if let Err(why) = battery_lifes::BatteryLife::new(
                system_info_model.id_system_information,
                battery_life.remaining_capacity,
                battery_life.remaining_time.as_secs() as i64,
            )
            .insert(&mut *db)
            .await
            {
                rocket::error!("Failed to insert battery life info: {why}.");
            }
        }

        for mount in info.mounts.iter() {
            if let Err(why) = filesystem_infos::FilesystemInfo::new(
                system_info_model.id_system_information,
                mount.files as i32,
                mount.files_total as i32,
                mount.free as i64,
                mount.avail as i64,
                mount.total as i64,
                mount.name_max as i32,
                &mount.fs_type,
                &mount.fs_mounted_from,
                &mount.fs_mounted_on,
            )
            .insert(&mut *db)
            .await
            {
                rocket::error!("Failed to insert mount info: {why}.");
            }
        }

        if let Err(why) =
            insert_network_data(system_info_model.id_system_information, &info, &mut *db).await
        {
            rocket::error!("Failed to insert network infos: {why}.");
        }

        if let Some(sock) = &info.socket_stats {
            if let Err(why) = socket_statistics::SocketStatistic::new(
                system_info_model.id_system_information,
                sock.tcp_sockets_in_use as i32,
                sock.tcp_sockets_orphaned as i32,
                sock.udp_sockets_in_use as i32,
                sock.tcp6_sockets_in_use as i32,
                sock.udp6_sockets_in_use as i32,
            )
            .insert(&mut *db)
            .await
            {
                rocket::error!("Failed to insert load averages: {why}.");
            }
        }

        rocket::info!("Inserted new system info for profile '{profile_id}'.");
        return Status::Ok;
    }

    Status::InternalServerError
}

async fn insert_network_data(
    id_system_info: i32,
    info: &Json<monitoring_core::models::SystemInformation>,
    db: &mut PoolConnection<Postgres>,
) -> Result<(), sqlx::Error> {
    for (interface, network) in info.networks.iter() {
        let network_model = networks::Network::new(id_system_info, &interface)
            .insert(&mut *db)
            .await?;

        for addr in network.addrs.iter() {
            if let Err(why) = network_addresses::NetworkAddress::new(
                network_model.id_network,
                &ip_to_string(addr.addr.clone()),
                &ip_to_string(addr.netmask.clone()),
            )
            .insert(&mut *db)
            .await
            {
                rocket::error!("Failed to insert mount info: {why}.");
            }
        }

        if let Some(netstat) = info.net_stats.get(&network.name) {
            if let Err(why) = network_statistics::NetworkStatistic::new(
                id_system_info,
                network_model.id_network,
                netstat.rx_bytes as i64,
                netstat.tx_bytes as i64,
                netstat.rx_packets as i64,
                netstat.tx_packets as i64,
                netstat.rx_errors as i64,
                netstat.tx_errors as i64,
            )
            .insert(&mut *db)
            .await
            {
                rocket::error!("Failed to insert network statistics: {why}.");
            }
        }
    }

    Ok(())
}

fn ip_to_string(ip: IpAddress) -> String {
    match ip {
        IpAddress::Empty => String::new(),
        IpAddress::Unsupported => String::from("Unsupported"),
        IpAddress::V4(v4) => v4.to_string(),
        IpAddress::V6(v6) => v6.to_string(),
    }
}

async fn insert_cpu_data(
    id_system_info: i32,
    info: &Json<monitoring_core::models::SystemInformation>,
    db: &mut PoolConnection<Postgres>,
) -> Result<(), sqlx::Error> {
    if let Some(cpu) = &info.cpu {
        let aggregate_load = cpu_loads::CpuLoad::new(
            cpu.aggregate_load.user,
            cpu.aggregate_load.nice,
            cpu.aggregate_load.system,
            cpu.aggregate_load.interrupt,
            cpu.aggregate_load.idle,
        )
        .insert(&mut *db)
        .await?;

        let cpu_info = cpu_informations::CpuInformation::new(
            id_system_info,
            cpu.temperature,
            aggregate_load.id_cpu_load,
        )
        .insert(&mut *db)
        .await?;

        for cpu_load in cpu.loads.iter() {
            let core_load = cpu_loads::CpuLoad::new(
                cpu_load.user,
                cpu_load.nice,
                cpu_load.system,
                cpu_load.interrupt,
                cpu_load.idle,
            )
            .insert(&mut *db)
            .await?;

            cpu_core_loads::CpuCoreLoad::new(cpu_info.id_cpu_information, core_load.id_cpu_load)
                .insert(&mut *db)
                .await?;
        }
    }

    Ok(())
}
