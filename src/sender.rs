use crate::models::SystemInfo;
use crate::config::Config;
use crate::crypto;
use reqwest::blocking::Client;
use std::time::Duration;
use log::{info, error, debug, warn};

pub fn send_to_backend(info: &SystemInfo, config: &Config) -> Result<(), String> {
    if !config.server.enabled {
        debug!("Backend communication disabled in config");
        return Ok(());
    }

    debug!("Loading API token from encrypted storage...");
    let api_token = crypto::load_token()
        .map_err(|e| {
            error!("Failed to load API token: {}", e);
            e
        })?;
    
    info!("Sending data to backend: {}", config.server.url);
    debug!("Agent ID: {}", info.agent_id);
    debug!("Hostname: {}", info.hostname);
    
    // Create HTTP client with timeout
    let client = Client::builder()
        .timeout(Duration::from_secs(config.server.timeout_seconds))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let auth_header = format!("Bearer {}", api_token);
    
    // Send POST request
    let response = client
        .post(&config.server.url)
        .header("Content-Type", "application/json")
        .header("Authorization", auth_header) 
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
        
        match status.as_u16() {
            401 => {
                error!("✗ Authentication failed (401 Unauthorized)");
                error!("   Check your API token in config.toml");
                error!("   Try re-registering: device-agent --register <new_token>");
            }
            403 => {
                error!("✗ Access forbidden (403 Forbidden)");
                error!("   Your token may have been revoked");
            }
            400 => {
                error!("✗ Bad request (400 Bad Request)");
                error!("   Error: {}", error_body);
            }
            500..=599 => {
                error!("✗ Server error ({} Server Error)", status);
                error!("   The backend server encountered an error");
            }
            _ => {
                error!("✗ Unexpected error (status {})", status);
            }
        }
        
        Err(format!("Backend error ({}): {}", status, error_body))
    }
}