use std::process::Command;

pub fn get_services() -> Vec<String> {
    let output = Command::new("launchctl")
        .args(&["list"])
        .output()
        .expect("Failed to execute launchctl");
    
    parse_services_output(output)
}

pub fn get_installed_application() -> Vec<String> {
    let output = Command::new("system_profiler")
        .args(&["SPApplicationsDataType"])
        .output()
        .expect("Failed to get applications");
    parse_applications_output(output)
}

fn parse_services_output(output: std::process::Output) -> Vec<String> {
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .skip(1) 
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                Some(parts[2].to_string())
            } else {
                None
            }
        })
        .collect()
}

fn parse_applications_output(output: std::process::Output) -> Vec<String> {
    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut apps = Vec::new();
    for line in output_str.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("_name:") {
            if let Some(app_name) = trimmed.strip_prefix("_name:") {
                apps.push(app_name.trim().to_string());
            }
        }
    }
    if apps.is_empty() {
        apps = get_apps_from_directory();
    } 
    apps
}

fn get_apps_from_directory() -> Vec<String> {
    let mut apps = Vec::new();
    if let Ok(entries) = std::fs::read_dir("/Applications") {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".app") {
                    apps.push(name.trim_end_matches(".app").to_string());
                }
            }
        }
    }
    apps
}