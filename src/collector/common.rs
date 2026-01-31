use sysinfo::System;
use crate::models::SystemInfo;

pub fn collect_basic_info() -> SystemInfo{
    let mut sys = System::new_all();
    sys.refresh_all();

    SystemInfo{
        hostname: get_hostname(),
        os_type: get_os_type(),
        os_version: get_os_version(&sys),
        cpu_info: get_cpu_info(&sys),
        memory_total: sys.total_memory(),
        memory_available: sys.available_memory(),
        ip_addresses: Vec::new(),
    }
}


fn get_hostname() -> String{
    System::host_name().unwrap_or_else(|| "Unknown".to_string())
}

fn get_os_type() -> String{
    std::env::consts::OS.to_string()
}

fn get_os_version(sys: &System) -> String{
    System::long_os_version().unwrap_or_else(|| "Unknown".to_string())
}

fn get_cpu_info(sys: &System) -> String{
    sys.cpus().first().map(|cpu| cpu.brand().to_string()).unwrap_or_else(|| "Unknown".to_string())
}