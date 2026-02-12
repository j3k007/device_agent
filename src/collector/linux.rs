use std::process::Command;
use log::{debug, warn};

/// Get running services on Linux (systemd)
pub fn get_services() -> Vec<String> {
    let mut services = Vec::new();
    
    debug!("Collecting Linux services...");
    
    // Get systemd services
    if let Ok(output) = Command::new("systemctl")
        .args(&["list-units", "--type=service", "--state=running", "--no-pager", "--no-legend"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        for line in stdout.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if !parts.is_empty() {
                let service_name = parts[0].trim_end_matches(".service");
                services.push(service_name.to_string());
            }
        }
    } else {
        warn!("Failed to get Linux services");
    }
    
    debug!("Found {} services", services.len());
    services
}

/// Get installed software on Linux
pub fn get_software() -> Vec<String> {
    let mut software = Vec::new();
    
    debug!("Collecting Linux packages...");
    
    // Try dpkg (Debian/Ubuntu)
    if let Ok(output) = Command::new("dpkg")
        .args(&["--get-selections"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        for line in stdout.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 && parts[1] == "install" {
                software.push(parts[0].to_string());
            }
        }
    }
    // Try rpm (RedHat/CentOS/Fedora)
    else if let Ok(output) = Command::new("rpm")
        .args(&["-qa", "--qf", "%{NAME}\n"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        for line in stdout.lines() {
            if !line.is_empty() {
                software.push(line.to_string());
            }
        }
    } else {
        warn!("Failed to get Linux packages");
    }
    
    software.sort();
    debug!("Found {} packages", software.len());
    software
}