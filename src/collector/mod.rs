// src/collector/mod.rs

pub mod common;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "windows")]
pub mod windows;

use crate::config::Config;
use crate::models::SystemInfo;
use crate::fingerprint;
use chrono::Utc;
use log::{error, debug, info};

/// Collect all system information including fingerprint
pub fn collect_all_info(config: &Config) -> SystemInfo {
    // Collect basic system info
    let basic = common::collect_basic_info(config);
    
    // Generate device fingerprint
    let device_fingerprint = match fingerprint::generate_fingerprint() {
        Ok(fp) => {
            debug!("Device fingerprint: {}...", &fp[..16]);
            fp
        }
        Err(e) => {
            error!("Failed to generate device fingerprint: {}", e);
            error!("Using fallback fingerprint based on hostname");
            // Fallback to hostname-based fingerprint
            use sha2::{Sha256, Digest};
            let mut hasher = Sha256::new();
            hasher.update(format!("fallback_{}", basic.hostname).as_bytes());
            format!("{:x}", hasher.finalize())
        }
    };

    // Collect services
    info!("Collecting services...");
    let services = get_services();
    info!("✓ Found {} services", services.len());
    
    // Collect installed software
    info!("Collecting installed software...");
    let installed_software = get_software();
    info!("✓ Found {} installed applications", installed_software.len());
    
    // Return system info (no services/software for Phase 2)
    SystemInfo {
        agent_id: config.agent.agent_id.clone(),
        agent_name: config.agent.agent_name.clone(),
        device_fingerprint,
        hostname: basic.hostname,
        os_type: basic.os_type,
        os_version: basic.os_version,
        cpu_info: basic.cpu_info,
        memory_total: basic.memory_total,
        memory_available: basic.memory_available,
        ip_addresses: basic.ip_addresses,
        services,
        installed_software,
        collected_at: Utc::now(),
    }
}

/// Get running services based on OS
fn get_services() -> Vec<String> {
    #[cfg(target_os = "macos")]
    {
        macos::get_services()
    }
    
    #[cfg(target_os = "linux")]
    {
        linux::get_services()
    }
    
    #[cfg(target_os = "windows")]
    {
        windows::get_services()
    }
    
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        Vec::new()
    }
}

/// Get installed software based on OS
fn get_software() -> Vec<String> {
    #[cfg(target_os = "macos")]
    {
        macos::get_software()
    }
    
    #[cfg(target_os = "linux")]
    {
        linux::get_software()
    }
    
    #[cfg(target_os = "windows")]
    {
        windows::get_software()
    }
    
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        Vec::new()
    }
}