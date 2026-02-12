// src/collector/common.rs

use crate::config::Config;
use sysinfo::{System};
use std::collections::HashMap;
use local_ip_address::local_ip;

/// Basic system information structure (internal use)
pub struct BasicInfo {
    pub hostname: String,
    pub os_type: String,
    pub os_version: String,
    pub cpu_info: String,
    pub memory_total: u64,
    pub memory_available: u64,
    pub ip_addresses: HashMap<String, String>,
}

/// Collect basic system information (platform-independent)
pub fn collect_basic_info(_config: &Config) -> BasicInfo {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    // Get hostname
    let hostname = System::host_name().unwrap_or_else(|| "unknown".to_string());
    
    // Get OS info
    let os_type = get_os_type();
    let os_version = System::os_version().unwrap_or_else(|| "unknown".to_string());
    
    // Get CPU info
    let cpu_info = if let Some(cpu) = sys.cpus().first() {
        cpu.brand().to_string()
    } else {
        "Unknown CPU".to_string()
    };
    
    // Get memory info
    let memory_total = sys.total_memory();
    let memory_available = sys.available_memory();
    
    // Get IP addresses
    let ip_addresses = get_ip_addresses();
    
    BasicInfo {
        hostname,
        os_type,
        os_version,
        cpu_info,
        memory_total,
        memory_available,
        ip_addresses,
    }
}

/// Get OS type as string
fn get_os_type() -> String {
    #[cfg(target_os = "macos")]
    return "macos".to_string();
    
    #[cfg(target_os = "linux")]
    return "linux".to_string();
    
    #[cfg(target_os = "windows")]
    return "windows".to_string();
    
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    return "unknown".to_string();
}

/// Get network IP addresses
fn get_ip_addresses() -> HashMap<String, String> {
    let mut addresses = HashMap::new();
    
    // Get local IP
    if let Ok(local_ip) = local_ip() {
        addresses.insert("local".to_string(), local_ip.to_string());
    }
    
    // Get all network interfaces
    #[cfg(target_os = "macos")]
    {
        if let Ok(output) = std::process::Command::new("ifconfig").output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let mut current_interface = String::new();
            
            for line in output_str.lines() {
                if !line.starts_with('\t') && !line.starts_with(' ') && line.contains(':') {
                    current_interface = line.split(':').next().unwrap_or("").to_string();
                } else if line.contains("inet ") && !current_interface.is_empty() {
                    if let Some(ip) = line.split_whitespace().nth(1) {
                        if ip != "127.0.0.1" {
                            addresses.insert(current_interface.clone(), ip.to_string());
                        }
                    }
                }
            }
        }
    }
    
    #[cfg(target_os = "linux")]
    {
        if let Ok(output) = std::process::Command::new("ip").args(&["addr"]).output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let mut current_interface = String::new();
            
            for line in output_str.lines() {
                if !line.starts_with(' ') && line.contains(':') {
                    let parts: Vec<&str> = line.split(':').collect();
                    if parts.len() >= 2 {
                        current_interface = parts[1].trim().to_string();
                    }
                } else if line.contains("inet ") && !current_interface.is_empty() {
                    if let Some(ip_part) = line.split_whitespace().nth(1) {
                        let ip = ip_part.split('/').next().unwrap_or("");
                        if ip != "127.0.0.1" && !current_interface.starts_with("lo") {
                            addresses.insert(current_interface.clone(), ip.to_string());
                        }
                    }
                }
            }
        }
    }
    
    #[cfg(target_os = "windows")]
    {
        if let Ok(output) = std::process::Command::new("ipconfig").output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let mut current_interface = String::new();
            
            for line in output_str.lines() {
                if !line.starts_with(' ') && line.contains(':') {
                    current_interface = line.trim().trim_end_matches(':').to_string();
                } else if line.contains("IPv4 Address") && !current_interface.is_empty() {
                    if let Some(ip) = line.split(':').nth(1) {
                        let ip = ip.trim();
                        if ip != "127.0.0.1" {
                            addresses.insert(current_interface.clone(), ip.to_string());
                        }
                    }
                }
            }
        }
    }
    
    addresses
}