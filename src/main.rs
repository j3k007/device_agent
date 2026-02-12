mod models;
mod collector;
mod config;
mod retry;
mod sender;
mod crypto;
mod fingerprint;

use collector::collect_all_info;
use config::Config;
use retry::retry_with_backoff;
use sender::send_to_backend; 
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::env;
use std::fs;
use log::{info, error, warn, debug};

fn main() {
    // ✅ Handle CLI arguments BEFORE loading config
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        match args[1].as_str() {
            "--init" | "-i" => {
                handle_init();
                return;
            }
            "--check-status" | "-s" => {
                handle_check_status();
                return;
            }
            "--register" | "-r" => {
                handle_register(&args);
                return;
            }
            "--unregister" | "-u" => {
                handle_unregister();
                return;
            }
            "--check-token" | "-c" => {
                handle_check_token();
                return;
            }
            "--help" | "-h" => {
                print_help();
                return;
            }
            _ => {
                eprintln!("✗ Unknown option: {}", args[1]);
                eprintln!("\nRun 'device-agent --help' for usage information");
                std::process::exit(1);
            }
        }
    }
    
    // ✅ Check if token is registered before starting
    if !crypto::has_token() {
        eprintln!("");
        eprintln!("✗ Error: No API token registered");
        eprintln!("");
        eprintln!("To get started:");
        eprintln!("1. Request registration: device-agent --init");
        eprintln!("2. Wait for admin approval");
        eprintln!("3. Check status: device-agent --check-status");
        eprintln!("");
        eprintln!("Alternative: If you have a token:");
        eprintln!("  device-agent --register <token>");
        eprintln!("");
        std::process::exit(1);
    }
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

// ✅ NEW: Check registration status
fn handle_check_status() {
    println!("");
    println!("=== Checking Registration Status ===");
    println!("");
    
    let config = match Config::load("config.toml") {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("✗ Error loading config: {}", e);
            std::process::exit(1);
        }
    };
    
    let status_url = config.server.url
        .replace("/api/heartbeat/", "/api/agents/register/")
        .replace("/heartbeat/", "/agents/register/") + &config.agent.agent_id + "/status/";
    
    println!("Checking status for: {}", config.agent.agent_id);
    println!("URL: {}", status_url);
    println!("");
    
    let client = reqwest::blocking::Client::new();
    match client.get(&status_url).send() {
        Ok(response) => {
            match response.json::<serde_json::Value>() {
                Ok(json) => {
                    let status = json.get("status").and_then(|v| v.as_str()).unwrap_or("unknown");
                    
                    match status {
                        "approved" => {
                            println!("✓ Status: APPROVED");
                            println!("");
                            
                            if let Some(token) = json.get("token").and_then(|v| v.as_str()) {
                                println!("Your device has been approved!");
                                println!("");
                                println!("Saving token automatically...");
                                
                                match crypto::save_token(token) {
                                    Ok(_) => {
                                        println!("✓ Token saved successfully!");
                                        println!("");
                                        println!("You can now start the agent:");
                                        println!("  device-agent");
                                    }
                                    Err(e) => {
                                        eprintln!("✗ Failed to save token: {}", e);
                                        println!("");
                                        println!("Please save manually:");
                                        println!("  device-agent --register {}", token);
                                    }
                                }
                            } else {
                                println!("Token already saved. You can start the agent:");
                                println!("  device-agent");
                            }
                        }
                        "pending" => {
                            println!("⏳ Status: PENDING APPROVAL");
                            println!("");
                            if let Some(msg) = json.get("message").and_then(|v| v.as_str()) {
                                println!("{}", msg);
                            }
                            println!("");
                            println!("Please wait for admin approval.");
                            println!("Check again later: device-agent --check-status");
                        }
                        "rejected" => {
                            println!("✗ Status: REJECTED");
                            println!("");
                            println!("Your registration was rejected.");
                            println!("Please contact your administrator for more information.");
                        }
                        "not_found" => {
                            println!("✗ Status: NOT FOUND");
                            println!("");
                            println!("No registration found for this device.");
                            println!("Please register first: device-agent --init");
                        }
                        _ => {
                            println!("Status: {}", status);
                            println!("{}", serde_json::to_string_pretty(&json).unwrap_or_default());
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to parse response: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("✗ Failed to check status: {}", e);
            eprintln!("");
            eprintln!("Make sure the backend is accessible.");
        }
    }
    
    println!("");
}

// ✅ NEW: Initialize and request registration
fn handle_init() {
    println!("");
    println!("=== Device Agent Initialization ===");
    println!("");
    
    // Load config
    let config = match Config::load("config.toml") {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("✗ Error loading config: {}", e);
            eprintln!("\nPlease create config.toml first:");
            eprintln!("  cp config.example.toml config.toml");
            std::process::exit(1);
        }
    };
    
    println!("Requesting registration for:");
    println!("  Agent ID:   {}", config.agent.agent_id);
    println!("  Agent Name: {}", config.agent.agent_name);
    println!("");
    
    // Collect system info
    println!("Collecting system information...");
    let info = collector::common::collect_basic_info(&config);
    
    // Generate fingerprint
    println!("Generating device fingerprint...");
    let device_fingerprint = match fingerprint::generate_fingerprint() {
        Ok(fp) => {
            println!("✓ Fingerprint: {}...", &fp[..16]);
            fp
        }
        Err(e) => {
            eprintln!("✗ Failed to generate fingerprint: {}", e);
            std::process::exit(1);
        }
    };
    
    println!("");
    
    // Prepare registration request
    let registration_data = serde_json::json!({
        "agent_id": config.agent.agent_id,
        "agent_name": config.agent.agent_name,
        "hostname": info.hostname,
        "os_type": info.os_type,
        "os_version": info.os_version,
        "device_fingerprint": device_fingerprint,
    });
    
    // Send registration request
    let registration_url = config.server.url.replace("/heartbeat/", "/agents/register/");
    
    println!("Sending registration request to:");
    println!("  {}", registration_url);
    println!("");
    
    let client = reqwest::blocking::Client::new();
    match client
        .post(&registration_url)
        .header("Content-Type", "application/json")
        .json(&registration_data)
        .send()
    {
        Ok(response) => {
            let status_code = response.status();
            
            match response.json::<serde_json::Value>() {
                Ok(json) => {
                    if status_code.is_success() || status_code.as_u16() == 202 {
                        println!("✓ Registration request submitted successfully!");
                        println!("");
                        if let Some(msg) = json.get("message").and_then(|v| v.as_str()) {
                            println!("{}", msg);
                        }
                        println!("");
                        println!("Next steps:");
                        println!("1. Admin will review your request in Django admin");
                        println!("2. Check status: device-agent --check-status");
                        println!("3. Once approved, token will be saved automatically");
                        println!("");
                    } else {
                        println!("✗ Registration failed ({})", status_code);
                        println!("");
                        if let Some(error) = json.get("error").and_then(|v| v.as_str()) {
                            println!("Error: {}", error);
                        }
                        if let Some(msg) = json.get("message").and_then(|v| v.as_str()) {
                            println!("{}", msg);
                        }
                        println!("");
                        std::process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("✗ Failed to parse response: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("");
            eprintln!("✗ Failed to send registration request:");
            eprintln!("  {}", e);
            eprintln!("");
            eprintln!("Make sure:");
            eprintln!("- Backend server is running");
            eprintln!("- URL in config.toml is correct");
            eprintln!("- Network connectivity is working");
            std::process::exit(1);
        }
    }
}

// ✅ NEW: Handle registration
fn handle_register(args: &[String]) {
    if args.len() < 3 {
        eprintln!("✗ Error: Missing token argument");
        eprintln!("");
        eprintln!("Usage:");
        eprintln!("  device-agent --register <api_token>");
        eprintln!("");
        eprintln!("Example:");
        eprintln!("  device-agent --register agt_xxxxxxxxxxxxxxxxxxx");
        eprintln!("");
        std::process::exit(1);
    }
    
    let token = &args[2];
    
    println!("");
    println!("=== Registering Device Agent ===");
    println!("");
    
    match crypto::save_token(token) {
        Ok(_) => {
            println!("");
            println!("✓ Registration successful!");
            println!("");
            println!("Your API token has been encrypted and stored securely.");
            println!("You can now start the agent:");
            println!("  device-agent");
            println!("");
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("");
            eprintln!("✗ Registration failed: {}", e);
            eprintln!("");
            std::process::exit(1);
        }
    }
}

// ✅ NEW: Handle unregistration
fn handle_unregister() {
    println!("");
    println!("=== Unregistering Device Agent ===");
    println!("");
    
    match crypto::delete_token() {
        Ok(_) => {
            println!("");
            println!("✓ Unregistration successful!");
            println!("");
            println!("Your API token has been deleted.");
            println!("To use the agent again, register with:");
            println!("  device-agent --register <token>");
            println!("");
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("");
            eprintln!("✗ Unregistration failed: {}", e);
            eprintln!("");
            std::process::exit(1);
        }
    }
}

// ✅ NEW: Check token status
fn handle_check_token() {
    println!("");
    println!("=== Token Status ===");
    println!("");
    
    if crypto::has_token() {
        println!("✓ API token is registered");
        println!("  Location: {}", crypto::get_token_location());
        println!("");
        
        // Try to load token to verify it's valid
        match crypto::load_token() {
            Ok(token) => {
                println!("✓ Token is valid and can be decrypted");
                println!("  Length: {} characters", token.len());
                println!("  Starts with: {}...", &token[..7]);
                println!("");
            }
            Err(e) => {
                eprintln!("✗ Token file exists but cannot be decrypted: {}", e);
                eprintln!("");
                eprintln!("Try re-registering:");
                eprintln!("  device-agent --register <token>");
                eprintln!("");
                std::process::exit(1);
            }
        }
        
        std::process::exit(0);
    } else {
        println!("✗ No API token found");
        println!("");
        println!("Register your device with:");
        println!("  device-agent --register <token>");
        println!("");
        std::process::exit(1);
    }
}

// ✅ NEW: Print help
fn print_help() {
    println!("");
    println!("Device Agent - System Monitoring Agent");
    println!("");
    println!("USAGE:");
    println!("  device-agent [OPTIONS]");
    println!("");
    println!("OPTIONS:");
    println!("  --register <token>, -r <token>    Register agent with API token");
    println!("  --unregister, -u                   Remove stored API token");
    println!("  --check-token, -c                  Check if token is registered");
    println!("  --help, -h                         Show this help message");
    println!("");
    println!("EXAMPLES:");
    println!("  # Register agent with token from Django admin");
    println!("  device-agent --register agt_xxxxxxxxxxxxxxxxxxx");
    println!("");
    println!("  # Start agent (runs continuously)");
    println!("  device-agent");
    println!("");
    println!("  # Check if token is registered");
    println!("  device-agent --check-token");
    println!("");
    println!("  # Unregister (delete token)");
    println!("  device-agent --unregister");
    println!("");
    println!("Get your API token from:");
    println!("  http://localhost:8000/admin/agents/agenttoken/");
    println!("");
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