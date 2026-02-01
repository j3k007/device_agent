use sysinfo::System;
use crate::models::SystemInfo;
use std::net::IpAddr;
use std::collections::HashMap;

pub fn collect_basic_info() -> SystemInfo {
    let mut sys = System::new_all();
    sys.refresh_all();

    SystemInfo {
        hostname: get_hostname(),
        os_type: get_os_type(),
        os_version: get_os_version(),
        cpu_info: get_cpu_info(&sys),
        memory_total: sys.total_memory(),
        memory_available: sys.available_memory(),
        ip_addresses: get_ip_addresses(),
        services: Vec::new(),
        installed_software: Vec::new(),
        windows_updates: None,
    }
}

fn get_hostname() -> String {
    System::host_name().unwrap_or_else(|| "unknown".to_string())
}

fn get_os_type() -> String {
    std::env::consts::OS.to_string()
}

// ✅ FIXED: Removed unused sys parameter since System::long_os_version() is static
fn get_os_version() -> String {
    System::long_os_version().unwrap_or_else(|| "Unknown".to_string())
}

fn get_cpu_info(sys: &System) -> String {
    sys.cpus()
        .first()
        .map(|cpu| cpu.brand().to_string())
        .unwrap_or_else(|| "Unknown CPU".to_string())
}

fn get_ip_addresses() -> HashMap<String, Vec<String>> {
    use local_ip_address::list_afinet_netifas;
    let mut interface_map: HashMap<String, (Option<String>, Vec<String>)> = HashMap::new();
    match list_afinet_netifas() {
        Ok(interfaces) => {
            for (interface_name, ip) in interfaces {
                let entry = interface_map.entry(interface_name.clone())
                    .or_insert((None, Vec::new()));
                
                match ip {
                    IpAddr::V4(ipv4) => {
                        if !ipv4.is_loopback() {
                            entry.0 = Some(ipv4.to_string());
                        }
                    }
                    IpAddr::V6(ipv6) => {
                        if !ipv6.is_loopback() && !is_link_local(&ipv6) {
                            entry.1.push(ipv6.to_string());
                        }
                    }
                }
            }
        }
        Err(_) => {}
    }
    let mut result = HashMap::new();
    for (_, (ipv4_opt, ipv6_list)) in interface_map {
        if let Some(ipv4) = ipv4_opt {
            result.insert(ipv4, ipv6_list);
        }
    }
    
    result
}

fn is_link_local(ipv6: &std::net::Ipv6Addr) -> bool {
    ipv6.segments()[0] == 0xfe80
}