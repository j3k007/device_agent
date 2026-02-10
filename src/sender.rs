use crate::models::SystemInfo;
use crate::config::Config;
use reqwest::blocking::Client;
use std::time::Duration;
use log::{info, error, debug, warn};

pub fn send_to_backend(info: &SystemInfo, config: &Config) -> Result<(), String> {
    if !config.server.enabled {
        debug!("Backend communication disabled in config");
        return Ok(());
    }
    
    info!("Sending data to backend: {}", config.server.url);
    debug!("Agent ID: {}", info.agent_id);
    debug!("Hostname: {}", info.hostname);
    
    // Create HTTP client with timeout
    let client = Client::builder()
        .timeout(Duration::from_secs(config.server.timeout_seconds))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;
    
    // Send POST request
    let response = client
        .post(&config.server.url)
        .header("Content-Type", "application/json")
        .json(&info)
        .send()
        .map_err(|e| format!("Failed to send request: {}", e))?;
    
    // Check response status
    let status = response.status();
    
    if status.is_success() {
        info!("✓ Data sent successfully (status: {})", status);
        
        // Log response body for debugging
        match response.text() {
            Ok(body) => {
                debug!("Response body: {}", body);
            }
            Err(e) => {
                warn!("Could not read response body: {}", e);
            }
        }
        
        Ok(())
    } else {
        let error_body = response
            .text()
            .unwrap_or_else(|_| "No error details available".to_string());
        
        error!("✗ Backend returned error (status {}): {}", status, error_body);
        Err(format!("Backend error ({}): {}", status, error_body))
    }
}