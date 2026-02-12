use std::process::Command;
use log::{debug, warn};

/// Get running services on Windows
pub fn get_services() -> Vec<String> {
    let mut services = Vec::new();
    
    debug!("Collecting Windows services...");
    
    // Get services using wmic
    if let Ok(output) = Command::new("wmic")
        .args(&["service", "where", "State='Running'", "get", "Name"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        for line in stdout.lines().skip(1) {
            let service = line.trim();
            if !service.is_empty() && service != "Name" {
                services.push(service.to_string());
            }
        }
    } else {
        warn!("Failed to get Windows services");
    }
    
    debug!("Found {} services", services.len());
    services
}

/// Get installed software on Windows
pub fn get_software() -> Vec<String> {
    let mut software = Vec::new();
    
    debug!("Collecting Windows programs...");
    
    // Get installed programs using wmic
    if let Ok(output) = Command::new("wmic")
        .args(&["product", "get", "Name"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        for line in stdout.lines().skip(1) {
            let program = line.trim();
            if !program.is_empty() && program != "Name" {
                software.push(program.to_string());
            }
        }
    } else {
        warn!("Failed to get Windows programs");
    }
    
    software.sort();
    debug!("Found {} programs", software.len());
    software
}