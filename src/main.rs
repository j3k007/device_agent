mod models;
mod collector;
mod config;

use collector::collect_all_info;
use config::Config;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::fs;
use log::{info, error, warn};

fn main() {
    // Load configuration
    let config = match Config::load("config.toml") {
        Ok(cfg) => {
            println!("✓ Configuration loaded from config.toml");
            cfg
        }
        Err(e) => {
            eprintln!("⚠ Warning: {}", e);
            eprintln!("⚠ Using default configuration");
            Config::default()
        }
    };

    // Ensure directories exist
    if let Err(e) = config.ensure_directories() {
        eprintln!("✗ Error: {}", e);
        std::process::exit(1);
    }

    // Initialize logger
    if let Err(e) = init_logger(&config) {
        eprintln!("✗ Error initializing logger: {}", e);
        std::process::exit(1);
    }

    info!("=== Device Agent Starting ===");
    info!("Agent ID: {}", config.agent.agent_id);
    info!("Agent Name: {}", config.agent.agent_name);
    info!("Collection Interval: {} seconds", config.collection.interval_seconds);
    info!("Output Directory: {}", config.output.output_directory);
    
    // Setup Ctrl+C handler
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    
    ctrlc::set_handler(move || {
        warn!("Received Ctrl+C signal, shutting down gracefully...");
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl+C handler");

    info!("Press Ctrl+C to stop");
    info!("");

    // Main collection loop
    let mut iteration = 0;
    
    while running.load(Ordering::SeqCst) {
        iteration += 1;
        
        info!("=== Collection Iteration #{} ===", iteration);
        
        // Collect system information
        match collect_system_data(&config) {
            Ok(info) => {
                info!("✓ System data collected successfully");
                
                // Save to file if enabled
                if config.output.save_to_file {
                    match save_to_file(&info, &config) {
                        Ok(filename) => {
                            info!("✓ Data saved to: {}", filename);
                        }
                        Err(e) => {
                            error!("✗ Failed to save file: {}", e);
                        }
                    }
                }
                
                // TODO: Send to backend (we'll add this later)
                // send_to_backend(&info, &config)?;
            }
            Err(e) => {
                error!("✗ Failed to collect data: {}", e);
            }
        }
        
        // Check if we should continue
        if !running.load(Ordering::SeqCst) {
            break;
        }
        
        // Wait for next collection
        info!("Waiting {} seconds until next collection...", config.collection.interval_seconds);
        info!("");
        
        // Sleep in small intervals to allow quick shutdown
        let sleep_interval = 1; // Check every second
        let total_sleep = config.collection.interval_seconds;
        
        for _ in 0..total_sleep {
            if !running.load(Ordering::SeqCst) {
                break;
            }
            thread::sleep(Duration::from_secs(sleep_interval));
        }
    }

    info!("=== Device Agent Stopped ===");
    info!("Total iterations: {}", iteration);
}

/// Collect system data with error handling
fn collect_system_data(config: &Config) -> Result<models::SystemInfo, String> {
    info!("Collecting system information...");
    
    let start_time = std::time::Instant::now();
    let info = collect_all_info(config);
    let elapsed = start_time.elapsed();
    
    info!("Collection completed in {:.2}s", elapsed.as_secs_f64());
    info!("  - Hostname: {}", info.hostname);
    info!("  - Services: {} items", info.services.len());
    info!("  - Software: {} items", info.installed_software.len());
    
    Ok(info)
}

/// Save collected data to JSON file
fn save_to_file(info: &models::SystemInfo, config: &Config) -> Result<String, String> {
    let timestamp = info.collected_at.format(&config.output.timestamp_format).to_string();
    let filename = format!("{}/system_info_{}.json", config.output.output_directory, timestamp);
    
    let json = serde_json::to_string_pretty(&info)
        .map_err(|e| format!("Failed to serialize data: {}", e))?;
    
    fs::write(&filename, json)
        .map_err(|e| format!("Failed to write file: {}", e))?;
    
    Ok(filename)
}

/// Initialize logging with console and file support
fn init_logger(config: &Config) -> Result<(), String> {
    // Parse log level
    let log_level = match config.logging.level.to_lowercase().as_str() {
        "trace" => log::LevelFilter::Trace,
        "debug" => log::LevelFilter::Debug,
        "info" => log::LevelFilter::Info,
        "warn" => log::LevelFilter::Warn,
        "error" => log::LevelFilter::Error,
        _ => log::LevelFilter::Info,
    };

    // Base configuration
    let mut dispatch = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log_level)
        // Filter out noisy dependencies
        .level_for("hyper", log::LevelFilter::Warn)
        .level_for("reqwest", log::LevelFilter::Warn);

    // Add console output if enabled
    if config.logging.console {
        dispatch = dispatch.chain(std::io::stdout());
    }

    // Add file output if enabled
    if config.logging.file {
        let log_file_path = format!(
            "{}/agent_{}.log",
            config.logging.log_directory,
            chrono::Utc::now().format("%Y%m%d")
        );
        
        match fern::log_file(&log_file_path) {
            Ok(file) => {
                dispatch = dispatch.chain(file);
                println!("✓ Logging to file: {}", log_file_path);
            }
            Err(e) => {
                eprintln!("⚠ Warning: Failed to open log file '{}': {}", log_file_path, e);
                eprintln!("  Continuing with console logging only");
            }
        }
    }

    // Apply the logging configuration
    dispatch.apply()
        .map_err(|e| format!("Failed to initialize logger: {}", e))?;

    Ok(())
}