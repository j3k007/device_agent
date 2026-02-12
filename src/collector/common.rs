use crate::config::Config;
use sysinfo::{System};
use std::collections::HashMap;

/// Basic system information structure (internal use)
pub struct BasicInfo {
    pub hostname: String,
    pub os_type: String,
    pub os_version: String,
    pub cpu_info: String,
    pub memory_total: u64,
    pub memory_available: u64,
    pub ip_addresses: HashMap<String, Vec<String>>,
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
    let memory_available = get_available_memory(&sys);
    
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

/// Get available memory (works across all platforms)
fn get_available_memory(sys: &System) -> u64 {
    let available = sys.available_memory();
    
    // If available_memory() returns 0 (common on macOS), calculate it
    if available == 0 {
        let total = sys.total_memory();
        let used = sys.used_memory();
        
        // Ensure we don't return negative values
        if total > used {
            return total - used;
        } else {
            // Fallback: try free_memory()
            return sys.free_memory();
        }
    }
    
    available
}

/// Get network IP addresses - IPv4 → [IPv6...]
fn get_ip_addresses() -> HashMap<String, Vec<String>> {
    let mut addresses: HashMap<String, Vec<String>> = HashMap::new();
    
    #[cfg(target_os = "macos")]
    {
        use std::collections::HashMap as TempMap;
        
        if let Ok(output) = std::process::Command::new("ifconfig").output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            
            // Store interface -> IPv4 mapping
            let mut interface_ipv4: TempMap<String, String> = TempMap::new();
            let mut current_interface = String::new();
            
            // First pass: collect all IPv4 addresses per interface
            for line in output_str.lines() {
                if !line.starts_with('\t') && !line.starts_with(' ') && line.contains(':') {
                    // New interface
                    current_interface = line.split(':').next().unwrap_or("").trim().to_string();
                } else if line.trim().starts_with("inet ") && !line.contains("inet6") {
                    // IPv4 address
                    if let Some(ip) = line.split_whitespace().nth(1) {
                        if ip != "127.0.0.1" && !ip.starts_with("169.254") && !current_interface.is_empty() {
                            interface_ipv4.insert(current_interface.clone(), ip.to_string());
                            addresses.entry(ip.to_string()).or_insert_with(Vec::new);
                        }
                    }
                }
            }
            
            // Reset for second pass
            current_interface.clear();
            
            // Second pass: collect all IPv6 addresses and associate with IPv4
            for line in output_str.lines() {
                if !line.starts_with('\t') && !line.starts_with(' ') && line.contains(':') {
                    // New interface
                    current_interface = line.split(':').next().unwrap_or("").trim().to_string();
                } else if line.trim().starts_with("inet6 ") {
                    // IPv6 address
                    if let Some(ipv6_part) = line.split_whitespace().nth(1) {
                        // Remove %interface suffix if present
                        let ipv6 = ipv6_part.split('%').next().unwrap_or(ipv6_part);
                        
                        // Filter out link-local (fe80::) and loopback (::1)
                        if !ipv6.starts_with("fe80") && ipv6 != "::1" && !current_interface.is_empty() {
                            // Find the IPv4 for this interface
                            if let Some(ipv4) = interface_ipv4.get(&current_interface) {
                                addresses.entry(ipv4.clone())
                                    .or_insert_with(Vec::new)
                                    .push(ipv6.to_string());
                            }
                        }
                    }
                }
            }
        }
    }
    
    #[cfg(target_os = "linux")]
    {
        use std::collections::HashMap as TempMap;
        
        if let Ok(output) = std::process::Command::new("ip").args(&["addr"]).output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            
            let mut interface_ipv4: TempMap<String, String> = TempMap::new();
            let mut current_interface = String::new();
            
            // First pass: collect IPv4 addresses
            for line in output_str.lines() {
                if !line.starts_with(' ') && line.contains(':') {
                    let parts: Vec<&str> = line.split(':').collect();
                    if parts.len() >= 2 {
                        current_interface = parts[1].trim().to_string();
                    }
                } else if line.trim().starts_with("inet ") && !line.contains("inet6") {
                    if let Some(ip_part) = line.split_whitespace().nth(1) {
                        let ip = ip_part.split('/').next().unwrap_or("");
                        if ip != "127.0.0.1" && !current_interface.starts_with("lo") && !current_interface.is_empty() {
                            interface_ipv4.insert(current_interface.clone(), ip.to_string());
                            addresses.entry(ip.to_string()).or_insert_with(Vec::new);
                        }
                    }
                }
            }
            
            // Reset for second pass
            current_interface.clear();
            
            // Second pass: collect IPv6 addresses
            for line in output_str.lines() {
                if !line.starts_with(' ') && line.contains(':') {
                    let parts: Vec<&str> = line.split(':').collect();
                    if parts.len() >= 2 {
                        current_interface = parts[1].trim().to_string();
                    }
                } else if line.trim().starts_with("inet6") {
                    if let Some(ip_part) = line.split_whitespace().nth(1) {
                        let ipv6 = ip_part.split('/').next().unwrap_or("");
                        
                        if !ipv6.starts_with("fe80") && ipv6 != "::1" && !current_interface.is_empty() {
                            if let Some(ipv4) = interface_ipv4.get(&current_interface) {
                                addresses.entry(ipv4.clone())
                                    .or_insert_with(Vec::new)
                                    .push(ipv6.to_string());
                            }
                        }
                    }
                }
            }
        }
    }
    
    #[cfg(target_os = "windows")]
    {
        use std::collections::HashMap as TempMap;
        
        if let Ok(output) = std::process::Command::new("ipconfig").output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            
            let mut interface_ipv4: TempMap<String, String> = TempMap::new();
            let mut current_interface = String::new();
            
            // First pass: collect IPv4 addresses
            for line in output_str.lines() {
                if !line.starts_with(' ') && line.contains(':') {
                    current_interface = line.trim().trim_end_matches(':').to_string();
                } else if line.contains("IPv4 Address") {
                    if let Some(ip) = line.split(':').nth(1) {
                        let ip = ip.trim().trim_start_matches(".")
                            .split('(').next().unwrap_or("").trim();
                        if ip != "127.0.0.1" && !current_interface.is_empty() {
                            interface_ipv4.insert(current_interface.clone(), ip.to_string());
                            addresses.entry(ip.to_string()).or_insert_with(Vec::new);
                        }
                    }
                }
            }
            
            // Reset for second pass
            current_interface.clear();
            
            // Second pass: collect IPv6 addresses
            for line in output_str.lines() {
                if !line.starts_with(' ') && line.contains(':') {
                    current_interface = line.trim().trim_end_matches(':').to_string();
                } else if line.contains("IPv6 Address") {
                    if let Some(addr_part) = line.split("IPv6 Address").nth(1) {
                        let ipv6 = addr_part.trim()
                            .trim_start_matches(':')
                            .trim_start_matches('.')
                            .split('(').next().unwrap_or("").trim();
                        
                        if !ipv6.is_empty() && !ipv6.starts_with("fe80") && ipv6 != "::1" && !current_interface.is_empty() {
                            if let Some(ipv4) = interface_ipv4.get(&current_interface) {
                                addresses.entry(ipv4.clone())
                                    .or_insert_with(Vec::new)
                                    .push(ipv6.to_string());
                            }
                        }
                    }
                }
            }
        }
    }
    
    addresses
}