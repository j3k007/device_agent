use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo{
    pub hostname: String,
    pub os_type: String,
    pub os_version: String,
    pub cpu_info: String,
    pub memory_total: u64,
    pub memory_available: u64,
    pub ip_addresses: Vec<String>,
}