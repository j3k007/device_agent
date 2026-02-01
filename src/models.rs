use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub hostname: String,
    pub os_type: String,
    pub os_version: String,
    pub cpu_info: String,
    pub memory_total: u64,
    pub memory_available: u64,
    pub ip_addresses: HashMap<String, Vec<String>>,
    pub services: Vec<String>,
    pub installed_software: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_updates: Option<Vec<String>>,
}