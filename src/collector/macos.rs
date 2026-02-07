use std::process::Command;

// ✅ ADD pub to make it public
pub fn get_services() -> Vec<String> {
    let output = Command::new("launchctl")
        .args(&["list"])
        .output()
        .expect("Failed to execute launchctl");
    
    parse_services_output(output)
}

// ✅ ADD pub to make it public
pub fn get_installed_software() -> Vec<String> {
    let output = Command::new("system_profiler")
        .args(&["SPApplicationsDataType"])
        .output()
        .expect("Failed to get applications");
    
    parse_applications_output(output)  // ✅ FIXED: Use correct function name
}

fn parse_services_output(output: std::process::Output) -> Vec<String> {
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .skip(1)  // Skip header
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                Some(parts[2].to_string())  // Get service name
            } else {
                None
            }
        })
        .collect()
}

// ✅ ADD this missing function
fn parse_applications_output(output: std::process::Output) -> Vec<String> {
    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut apps = Vec::new();
    
    // Parse system_profiler output for application names
    // Look for lines with "_name:" which contain app names
    for line in output_str.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("_name:") {
            if let Some(app_name) = trimmed.strip_prefix("_name:") {
                let name = app_name.trim().trim_matches('"');
                if !name.is_empty() {
                    apps.push(name.to_string());
                }
            }
        }
    }
    
    // If no apps found via system_profiler, try listing /Applications
    if apps.is_empty() {
        apps = get_apps_from_directory();
    }
    
    apps
}

// ✅ ADD helper function
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