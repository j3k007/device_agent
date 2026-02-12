use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub collected_at: DateTime<Utc>,
    pub agent_id: String,
    pub agent_name: String,
    pub device_fingerprint: String,
    pub hostname: String,
    pub os_type: String,
    pub os_version: String,
    pub cpu_info: String,
    pub memory_total: u64,
    pub memory_available: u64,
    // Network Info - IP → [IPv6 addresses]
    pub ip_addresses: HashMap<String, Vec<String>>,
    // Services - list of service names
    pub services: Vec<String>,
    // Installed Software - list of app names
    pub installed_software: Vec<String>,
}