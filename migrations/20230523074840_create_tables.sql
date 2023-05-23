CREATE TABLE IF NOT EXISTS device_profiles (
    id_device_profile SERIAL PRIMARY KEY,
    device_name VARCHAR(255) NOT NULL,
    profile_key VARCHAR(500) NOT NULL,
    create_user BIGINT NOT NULL,
    create_date TIMESTAMP NOT NULL,
    modify_user BIGINT,
    modify_date TIMESTAMP
);

CREATE TABLE IF NOT EXISTS error_logs (
    id_error_log SERIAL PRIMARY KEY,
    device_profile_id INT NOT NULL,
    message TEXT NOT NULL,
    create_date TIMESTAMP NOT NULL,
    FOREIGN KEY (device_profile_id) REFERENCES device_profiles(id_device_profile)
);

CREATE TABLE IF NOT EXISTS system_informations (
    id_system_information SERIAL PRIMARY KEY,
    device_profile_id INT NOT NULL,
    hostname VARCHAR(255) NOT NULL,
    uptime INT NOT NULL,
    boot_time TIMESTAMP NOT NULL,
    create_date TIMESTAMP NOT NULL,
    FOREIGN KEY (device_profile_id) REFERENCES device_profiles(id_device_profile)
);

CREATE TABLE IF NOT EXISTS os_infos (
    id_os_info SERIAL PRIMARY KEY,
    system_information_id INT NOT NULL,
    os_type VARCHAR(255) NOT NULL,
    version VARCHAR(50) NOT NULL,
    edition VARCHAR(255),
    codename VARCHAR(255),
    bitness VARCHAR(50) NOT NULL,
    architecture VARCHAR(255),
    FOREIGN KEY (system_information_id) REFERENCES system_informations(id_system_information)
);

CREATE TABLE IF NOT EXISTS cpu_loads (
    id_cpu_load SERIAL PRIMARY KEY,
    "user" FLOAT NOT NULL,
    nice FLOAT NOT NULL,
    system FLOAT NOT NULL,
    interrupt FLOAT NOT NULL,
    idle FLOAT NOT NULL
);

CREATE TABLE IF NOT EXISTS cpu_informations (
    id_cpu_information SERIAL PRIMARY KEY,
    system_information_id INT NOT NULL,
    temperature FLOAT NOT NULL,
    aggregate_load_id INT NOT NULL,
    FOREIGN KEY (system_information_id) REFERENCES system_informations(id_system_information),
    FOREIGN KEY (aggregate_load_id) REFERENCES cpu_loads(id_cpu_load)
);

CREATE TABLE IF NOT EXISTS cpu_core_loads (
    id_cpu_core_load SERIAL PRIMARY KEY,
    cpu_information_id INT NOT NULL,
    cpu_load_id INT NOT NULL,
    FOREIGN KEY (cpu_load_id) REFERENCES cpu_loads(id_cpu_load),
    FOREIGN KEY (cpu_information_id) REFERENCES cpu_informations(id_cpu_information)
);

CREATE TABLE IF NOT EXISTS load_averages (
    id_load_average SERIAL PRIMARY KEY,
    system_information_id INT NOT NULL,
    one INT NOT NULL,
    five INT NOT NULL,
    fifteen INT NOT NULL,
    FOREIGN KEY (system_information_id) REFERENCES system_informations(id_system_information)
);

CREATE TABLE IF NOT EXISTS memory_infos (
    id_memory_info SERIAL PRIMARY KEY,
    system_information_id INT NOT NULL,
    free BIGINT NOT NULL,
    total BIGINT NOT NULL,
    FOREIGN KEY (system_information_id) REFERENCES system_informations(id_system_information)
);

CREATE TABLE IF NOT EXISTS swap_infos (
    id_swap_info SERIAL PRIMARY KEY,
    system_information_id INT NOT NULL,
    free BIGINT NOT NULL,
    total BIGINT NOT NULL,
    FOREIGN KEY (system_information_id) REFERENCES system_informations(id_system_information)
);

CREATE TABLE IF NOT EXISTS battery_lifes (
    id_battery_life SERIAL PRIMARY KEY,
    system_information_id INT NOT NULL,
    remaining_capacity FLOAT NOT NULL,
    remaining_time BIGINT NOT NULL,
    FOREIGN KEY (system_information_id) REFERENCES system_informations(id_system_information)
);

CREATE TABLE IF NOT EXISTS filesystem_infos (
    id_filesystem_info SERIAL PRIMARY KEY,
    system_information_id INT NOT NULL,
    files INT NOT NULL,
    files_total INT NOT NULL,
    free BIGINT NOT NULL,
    avail BIGINT NOT NULL,
    total BIGINT NOT NULL,
    name_max INT NOT NULL,
    fs_type VARCHAR(255) NOT NULL,
    fs_mounted_from VARCHAR(255) NOT NULL,
    fs_mounted_on VARCHAR(255) NOT NULL,
    FOREIGN KEY (system_information_id) REFERENCES system_informations(id_system_information)
);

CREATE TABLE IF NOT EXISTS networks (
    id_network SERIAL PRIMARY KEY,
    system_information_id INT NOT NULL,
    name VARCHAR(255) NOT NULL,
    FOREIGN KEY (system_information_id) REFERENCES system_informations(id_system_information)
);

CREATE TABLE IF NOT EXISTS network_addresses (
    id_network_address SERIAL PRIMARY KEY,
    network_id INT NOT NULL,
    address VARCHAR(255) NOT NULL,
    netmask VARCHAR(255) NOT NULL,
    FOREIGN KEY (network_id) REFERENCES networks(id_network)
);

CREATE TABLE IF NOT EXISTS networks_statistics (
    id_network_statistics SERIAL PRIMARY KEY,
    system_information_id INT NOT NULL,
    rx_bytes BIGINT NOT NULL,
    tx_bytes BIGINT NOT NULL,
    rx_packages BIGINT NOT NULL,
    tx_packages BIGINT NOT NULL,
    rx_errors BIGINT NOT NULL,
    tx_errors BIGINT NOT NULL,
    FOREIGN KEY (system_information_id) REFERENCES system_informations(id_system_information)
);

CREATE TABLE IF NOT EXISTS networks_statistics (
    id_network_statistics SERIAL PRIMARY KEY,
    system_information_id INT NOT NULL,
    tcp_sockets_in_use INT NOT NULL,
    tcp_sockets_orphaned INT NOT NULL,
    udp_sockets_in_use INT NOT NULL,
    tcp6_sockets_in_use INT NOT NULL,
    udp6_sockets_in_use INT NOT NULL,
    FOREIGN KEY (system_information_id) REFERENCES system_informations(id_system_information)
);