use std::process::Command;

// ✅ ADD pub
pub fn get_services() -> Vec<String> {
    let output = Command::new("systemctl")
        .args(&["list-units", "--type=service", "--all"])
        .output()
        .expect("Failed to execute systemctl");
    
    parse_services_output(output)
}

// ✅ ADD pub
pub fn get_installed_software() -> Vec<String> {
    let output = Command::new("dpkg")
        .args(&["--list"])
        .output();
    
    match output {
        Ok(out) => parse_dpkg_output(out),
        Err(_) => try_rpm_packages()
    }
}

fn parse_services_output(output: std::process::Output) -> Vec<String> {
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn parse_dpkg_output(output: std::process::Output) -> Vec<String> {
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| {
            if line.starts_with("ii") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    return Some(parts[1].to_string());
                }
            }
            None
        })
        .collect()
}

fn try_rpm_packages() -> Vec<String> {
    match Command::new("rpm")
        .args(&["-qa"])
        .output() {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout)
                .lines()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        }
        _ => Vec::new()
    }
}