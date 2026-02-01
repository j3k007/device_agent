use std::process::Command;

pub fn get_windows_services() -> Vec<String> {
    let output = Command::new("powershell")
        .args(&["-Command", "Get-Service | Select-Object Name, Status"])  
        .output()
        .expect("Failed to execute PowerShell");
    
    parse_services_output(output)
}

pub fn get_installed_software() -> Vec<String> {
    let output = Command::new("powershell")
        .args(&["-Command", 
            "Get-ItemProperty HKLM:\\Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\* | Select-Object DisplayName"])
        .output()
        .expect("Failed to get installed software");
    
    parse_software_output(output)
}

pub fn get_windows_updates() -> Vec<String> {
    let output = Command::new("powershell")
        .args(&["-Command", "Get-HotFix | Select-Object HotFixID, InstalledOn"])
        .output()
        .expect("Failed to get Windows updates");
    
    parse_updates_output(output)
}

fn parse_services_output(output: std::process::Output) -> Vec<String> {
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .skip(2)
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn parse_software_output(output: std::process::Output) -> Vec<String> {
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .skip(2)
        .filter_map(|line| {
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with('-') {
                Some(trimmed.to_string())
            } else {
                None
            }
        })
        .collect()
}

fn parse_updates_output(output: std::process::Output) -> Vec<String> {
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .skip(2)
        .filter_map(|line| {
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with('-') {
                Some(trimmed.to_string())
            } else {
                None
            }
        })
        .collect()
}