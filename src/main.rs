mod models;

use models::SystemInfo;

fn main() {
    let info = SystemInfo{
        hostname: "test-machine".to_string(),
        os_type: "Mac".to_string(),
        os_version: "10".to_string(),
        cpu_info: "apple silicon".to_string(),
        memory_total: 16000000000,
        memory_available: 8000000000,
        ip_addresses: vec!["192.168.1.100".to_string()],
    };
    let json = serde_json::to_string_pretty(&info).unwrap();
    println!("{}", json);
}
