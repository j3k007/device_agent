mod models;
mod collector;
mod config;
mod retry;
mod sender;

use collector::collect_all_info;
use config::Config;
use retry::retry_with_backoff;
use sender::send_to_backend; 
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::fs;
use log::{info, error, warn, debug};

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
    let mut successful_collections = 0;
    let mut failed_collections = 0;
    while running.load(Ordering::SeqCst) {
        iteration += 1;
        
        info!("=== Collection Iteration #{} ===", iteration);
        
        // ✅ UPDATED: Use retry logic
        match retry_with_backoff(
            "collect_and_save",
            &retry::RetryConfig {
                max_retries: config.retry.max_retries,
                initial_delay_ms: config.retry.initial_delay_ms,
                max_delay_ms: config.retry.max_delay_ms,
            },
            || collect_and_save(&config),
        ) {
            Ok(_) => {
                successful_collections += 1;
                info!("✓ Collection and save completed successfully");
            }
            Err(e) => {
                failed_collections += 1;
                error!("✗ All retry attempts failed: {}", e);
                warn!("Will try again in next collection cycle");
            }
        }
        
        debug!("Statistics: Total={}, Successful={}, Failed={}", 
            iteration, successful_collections, failed_collections);
        
        if !running.load(Ordering::SeqCst) {
            break;
        }
        
        info!("Waiting {} seconds until next collection...", config.collection.interval_seconds);
        info!("");
        
        sleep_with_interrupt(&running, config.collection.interval_seconds);
    }

    info!("=== Device Agent Stopped ===");
    info!("Total iterations: {}", iteration);
    info!("Successful collections: {}", successful_collections);
    info!("Failed collections: {}", failed_collections);
}

// ✅ NEW: Combined collect and save with proper error handling
fn collect_and_save(config: &Config) -> Result<(), String> {
    // Collect data
    let info = collect_system_data(config)?;
    
    // Save to file if enabled
    if config.output.save_to_file {
        save_to_file(&info, config)?;
    }
    // ✅ NEW: Send to backend if enabled
    if config.server.enabled {
        send_to_backend(&info, config)?;
    }
    
    Ok(())
}

fn collect_system_data(config: &Config) -> Result<models::SystemInfo, String> {
    debug!("Starting system information collection");
    
    let start_time = std::time::Instant::now();
    let info = collect_all_info(config);
    let elapsed = start_time.elapsed();
    
    info!("Collection completed in {:.2}s", elapsed.as_secs_f64());
    info!("  - Hostname: {}", info.hostname);
    info!("  - OS: {} {}", info.os_type, info.os_version);
    info!("  - Services: {} items", info.services.len());
    info!("  - Software: {} items", info.installed_software.len());
    
    Ok(info)
}

fn save_to_file(info: &models::SystemInfo, config: &Config) -> Result<String, String> {
    debug!("Preparing to save data to file");
    
    let timestamp = info.collected_at.format(&config.output.timestamp_format).to_string();
    let filename = format!("{}/system_info_{}.json", config.output.output_directory, timestamp);
    
    let json = serde_json::to_string_pretty(&info)
        .map_err(|e| format!("Failed to serialize data: {}", e))?;
    
    fs::write(&filename, json)
        .map_err(|e| format!("Failed to write file: {}", e))?;
    
    info!("✓ Data saved to: {}", filename);
    Ok(filename)
}

fn sleep_with_interrupt(running: &Arc<AtomicBool>, seconds: u64) {
    for _ in 0..seconds {
        if !running.load(Ordering::SeqCst) {
            debug!("Sleep interrupted by shutdown signal");
            break;
        }
        thread::sleep(Duration::from_secs(1));
    }
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