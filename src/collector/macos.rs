use std::process::Command;
use log::{debug, warn};

/// Get running services on macOS (launchd services)
pub fn get_services() -> Vec<String> {
    let mut services = Vec::new();
    
    debug!("Collecting macOS services...");
    
    // Get user services
    if let Ok(output) = Command::new("launchctl")
        .args(&["list"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        
        for line in stdout.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let service_name = parts[2];
                if !service_name.is_empty() {
                    services.push(service_name.to_string());
                }
            }
        }
    } else {
        warn!("Failed to get macOS services");
    }
    
    debug!("Found {} services", services.len());
    services
}

/// Get installed applications on macOS
pub fn get_software() -> Vec<String> {
    let mut software = Vec::new();
    
    debug!("Collecting macOS applications...");
    
    // Get applications from /Applications
    if let Ok(entries) = std::fs::read_dir("/Applications") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("app") {
                if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                    software.push(name.to_string());
                }
            }
        }
    }
    
    // Get applications from ~/Applications
    if let Ok(home) = std::env::var("HOME") {
        let user_apps = format!("{}/Applications", home);
        if let Ok(entries) = std::fs::read_dir(user_apps) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("app") {
                    if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                        if !software.contains(&name.to_string()) {
                            software.push(name.to_string());
                        }
                    }
                }
            }
        }
    }
    
    software.sort();
    debug!("Found {} applications", software.len());
    software
}