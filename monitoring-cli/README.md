# monitoring-cli

Commandline tool for retrieving system information to monitor devices. 
Sends the data to a defined server with a given `profile id`, `api-key` and `profile-key`.

## Features:

Can collect the following informations:
 - General information: 
    - Hostname
    - Uptime
    - Boottime
 - Cpu:
    - Temp
    - load per core
    - aggregated load
 - Memory: 
    - free
    - total
 - Filesystem:
    - Mounts
    - File amount, space and more per mount.
 - OS: 
    - Version
    - Type
    - Bitness
    - Edition
    - Architecture
 - Network:
    - Interfaces 
        - Netmask
        - Address
    - Network statistics
    - Socket statistics
 - Swap:
    - free
    - total

## Configuration:

```
monitoring-cli run service  // runs the cli in a loops so you have constant monitoring.
    -c --cpu                // Includes/Excludes CPU infos
    -m --memory             // Includes/Excludes memory infos
    -o --os                 // Includes/Excludes os infos
    -n --network            // Includes/Excludes network infos
    -f --filesystem         // Includes/Excludes filesystem infos
    -s --swap               // Includes/Excludes swap infos
    -i --interval <u32>     // Sleep interval in seconds.
```

### Examples: 
 - `monitoring-cli run service -s`
    - Runs the cli in a loop for all features except the swap informations from being collected.
 - `monitoring-cli run service -m -c`
    - Runs the cli in a loop collecting only memory and cpu informations.

```
monitoring-cli run normal   // runs the cli for a single time collecting data.
    -c --cpu                // Includes/Excludes CPU infos
    -m --memory             // Includes/Excludes memory infos
    -o --os                 // Includes/Excludes os infos
    -n --network            // Includes/Excludes network infos
    -f --filesystem         // Includes/Excludes filesystem infos
    -s --swap               // Includes/Excludes swap infos
```

### Examples: 
 - `monitoring-cli run normal -s`
    - Runs the cli a single time for all features except the swap informations from being collected.
 - `monitoring-cli run normal -m -c`
    - Runs the cli a single time collecting only memory and cpu informations.

```
monitoring-cli configure    // Changes the cli configuration for future runs. 
    -a --api-key <api-key> 
    -p --profile-key <profile-key>
    -i --id <id>
    -s --server-url <server-url>
```

### Examples:
 - `monitoring-cli configure -a this_is_a_bad_key`
    - Configures the api key to `this_is_a_bad_key`.
 - `monitoring-cli configure --id 1`
    - Configures the profile id of the cli