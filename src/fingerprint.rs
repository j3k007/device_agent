use std::process::Command;
use log::{debug, error, warn};
use sha2::{Sha256, Digest};

/// Generate a unique device fingerprint
/// 
/// This combines multiple hardware identifiers to create a unique,
/// stable fingerprint that identifies this specific device.
pub fn generate_fingerprint() -> Result<String, String> {
    debug!("Generating device fingerprint");
    
    let mut components = Vec::new();
    
    // Get hardware components based on OS
    #[cfg(target_os = "macos")]
    {
        components.extend(get_macos_components()?);
    }
    
    #[cfg(target_os = "linux")]
    {
        components.extend(get_linux_components()?);
    }
    
    #[cfg(target_os = "windows")]
    {
        components.extend(get_windows_components()?);
    }
    
    if components.is_empty() {
        return Err("Failed to collect any hardware identifiers".to_string());
    }
    
    // Combine components and hash
    let combined = components.join("|");
    let mut hasher = Sha256::new();
    hasher.update(combined.as_bytes());
    let result = hasher.finalize();
    let fingerprint = format!("{:x}", result);
    
    debug!("Device fingerprint generated: {}...", &fingerprint[..16]);
    debug!("Components used: {}", components.len());
    
    Ok(fingerprint)
}

#[cfg(target_os = "macos")]
fn get_macos_components() -> Result<Vec<String>, String> {
    let mut components = Vec::new();
    
    // Hardware UUID (most stable identifier on macOS)
    if let Ok(output) = Command::new("system_profiler")
        .args(&["SPHardwareDataType"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        // Extract Hardware UUID
        for line in stdout.lines() {
            if line.contains("Hardware UUID") {
                if let Some(uuid) = line.split(':').nth(1) {
                    let uuid = uuid.trim();
                    if !uuid.is_empty() {
                        components.push(format!("hw_uuid:{}", uuid));
                        debug!("Found Hardware UUID");
                    }
                }
            }
            // Also get Serial Number
            if line.contains("Serial Number") {
                if let Some(serial) = line.split(':').nth(1) {
                    let serial = serial.trim();
                    if !serial.is_empty() && serial != "(system)" {
                        components.push(format!("serial:{}", serial));
                        debug!("Found Serial Number");
                    }
                }
            }
        }
    }
    
    // MAC address of en0 (primary network interface)
    if let Ok(output) = Command::new("ifconfig")
        .arg("en0")
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if line.contains("ether") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    components.push(format!("mac:{}", parts[1]));
                    debug!("Found MAC address");
                }
            }
        }
    }
    
    if components.is_empty() {
        return Err("Failed to collect hardware identifiers on macOS".to_string());
    }
    
    Ok(components)
}

#[cfg(target_os = "linux")]
fn get_linux_components() -> Result<Vec<String>, String> {
    let mut components = Vec::new();
    
    // Machine ID (stable system identifier)
    if let Ok(machine_id) = std::fs::read_to_string("/etc/machine-id") {
        let machine_id = machine_id.trim();
        if !machine_id.is_empty() {
            components.push(format!("machine_id:{}", machine_id));
            debug!("Found machine-id");
        }
    } else if let Ok(machine_id) = std::fs::read_to_string("/var/lib/dbus/machine-id") {
        let machine_id = machine_id.trim();
        if !machine_id.is_empty() {
            components.push(format!("machine_id:{}", machine_id));
            debug!("Found dbus machine-id");
        }
    }
    
    // DMI Product UUID
    if let Ok(uuid) = std::fs::read_to_string("/sys/class/dmi/id/product_uuid") {
        let uuid = uuid.trim();
        if !uuid.is_empty() {
            components.push(format!("product_uuid:{}", uuid));
            debug!("Found product UUID");
        }
    }
    
    // Board Serial
    if let Ok(serial) = std::fs::read_to_string("/sys/class/dmi/id/board_serial") {
        let serial = serial.trim();
        if !serial.is_empty() && serial != "None" {
            components.push(format!("board_serial:{}", serial));
            debug!("Found board serial");
        }
    }
    
    // MAC address
    if let Ok(entries) = std::fs::read_dir("/sys/class/net") {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name() {
                let name_str = name.to_string_lossy();
                // Skip virtual interfaces
                if !name_str.starts_with("lo") && !name_str.starts_with("docker") && !name_str.starts_with("veth") {
                    let mac_path = path.join("address");
                    if let Ok(mac) = std::fs::read_to_string(mac_path) {
                        let mac = mac.trim();
                        if !mac.is_empty() && mac != "00:00:00:00:00:00" {
                            components.push(format!("mac:{}:{}", name_str, mac));
                            debug!("Found MAC address for {}", name_str);
                            break; // Just use first physical interface
                        }
                    }
                }
            }
        }
    }
    
    if components.is_empty() {
        return Err("Failed to collect hardware identifiers on Linux".to_string());
    }
    
    Ok(components)
}

#[cfg(target_os = "windows")]
fn get_windows_components() -> Result<Vec<String>, String> {
    let mut components = Vec::new();
    
    // Get Windows UUID using WMIC
    if let Ok(output) = Command::new("wmic")
        .args(&["csproduct", "get", "UUID"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines().skip(1) {
            let uuid = line.trim();
            if !uuid.is_empty() && uuid != "UUID" {
                components.push(format!("uuid:{}", uuid));
                debug!("Found Windows UUID");
            }
        }
    }
    
    // Get Windows Serial Number
    if let Ok(output) = Command::new("wmic")
        .args(&["bios", "get", "serialnumber"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines().skip(1) {
            let serial = line.trim();
            if !serial.is_empty() && serial != "SerialNumber" {
                components.push(format!("serial:{}", serial));
                debug!("Found BIOS serial");
            }
        }
    }
    
    // Get MAC address using getmac
    if let Ok(output) = Command::new("getmac")
        .arg("/fo")
        .arg("list")
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if line.contains("Physical Address") {
                if let Some(mac) = line.split(':').nth(1) {
                    let mac = mac.trim();
                    if !mac.is_empty() {
                        components.push(format!("mac:{}", mac));
                        debug!("Found MAC address");
                        break;
                    }
                }
            }
        }
    }
    
    if components.is_empty() {
        return Err("Failed to collect hardware identifiers on Windows".to_string());
    }
    
    Ok(components)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fingerprint_generation() {
        let fingerprint = generate_fingerprint();
        assert!(fingerprint.is_ok(), "Fingerprint generation should succeed");
        
        let fp = fingerprint.unwrap();
        assert_eq!(fp.len(), 64, "SHA256 hash should be 64 hex characters");
        
        // Fingerprint should be deterministic
        let fp2 = generate_fingerprint().unwrap();
        assert_eq!(fp, fp2, "Fingerprint should be stable across calls");
    }
}