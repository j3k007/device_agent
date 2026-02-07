use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub collection: CollectionConfig,
    pub output: OutputConfig,
    pub logging: LoggingConfig,
    pub agent: AgentConfig,
    pub retry: RetryConfig, 
}

#[derive(Debug, Deserialize, Clone)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub initial_delay_ms: u64,
    pub max_delay_ms: u64,
}


#[derive(Debug, Deserialize, Clone)]
pub struct CollectionConfig {
    pub interval_seconds: u64,
    pub include_services: bool,
    pub include_software: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OutputConfig {
    pub output_directory: String,
    pub save_to_file: bool,
    pub timestamp_format: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub console: bool,
    pub file: bool,
    pub log_directory: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AgentConfig {
    pub agent_id: String,
    pub agent_name: String,
}

impl Config {
    /// Load configuration from file
    pub fn load(path: &str) -> Result<Self, String> {
        // Check if file exists
        if !Path::new(path).exists() {
            return Err(format!(
                "Configuration file not found: {}\n\
                Please copy config.example.toml to config.toml and customize it.",
                path
            ));
        }

        // Read file
        let contents = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;

        // Parse TOML
        let config: Config = toml::from_str(&contents)
            .map_err(|e| format!("Failed to parse config file: {}", e))?;

        Ok(config)
    }

    /// Create default configuration
    pub fn default() -> Self {
        Config {
            collection: CollectionConfig {
                interval_seconds: 300,
                include_services: true,
                include_software: true,
            },
            output: OutputConfig {
                output_directory: "./data".to_string(),
                save_to_file: true,
                timestamp_format: "%Y%m%d_%H%M%S".to_string(),
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                console: true,
                file: true,
                log_directory: "./logs".to_string(),
            },
            agent: AgentConfig {
                agent_id: "agent-001".to_string(),
                agent_name: "Device Agent".to_string(),
            },
            retry: RetryConfig {  // ✅ NEW
                max_retries: 5,
                initial_delay_ms: 1000,
                max_delay_ms: 60000,
            },
        }
    }

    /// Ensure required directories exist
    pub fn ensure_directories(&self) -> Result<(), String> {
        // Create output directory
        if self.output.save_to_file {
            fs::create_dir_all(&self.output.output_directory)
                .map_err(|e| format!("Failed to create output directory: {}", e))?;
        }

        // Create log directory
        if self.logging.file {
            fs::create_dir_all(&self.logging.log_directory)
                .map_err(|e| format!("Failed to create log directory: {}", e))?;
        }

        Ok(())
    }
}