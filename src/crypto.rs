use aes_gcm::{
    aead::{Aead, KeyInit, OsRng, AeadCore},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose, Engine as _};
use std::fs;
use std::path::Path;
use log::{info, debug};

const TOKEN_FILE: &str = "/usr/local/etc/device-agent/.token";
const KEY_FILE: &str = "/usr/local/etc/device-agent/.key";

// For development on non-installed systems
const DEV_TOKEN_FILE: &str = "./.token";
const DEV_KEY_FILE: &str = "./.key";

/// Generate a machine-specific encryption key
fn generate_machine_key() -> Result<Vec<u8>, String> {
    // In production, derive from machine-specific data
    // For now, generate and save a random key
    let key = Aes256Gcm::generate_key(&mut OsRng);
    Ok(key.to_vec())
}

/// Get or create encryption key
fn get_encryption_key() -> Result<Vec<u8>, String> {
    let key_path = if Path::new(KEY_FILE).parent().map(|p| p.exists()).unwrap_or(false) {
        KEY_FILE
    } else {
        DEV_KEY_FILE
    };
    
    if Path::new(key_path).exists() {
        // Read existing key
        let key_data = fs::read(key_path)
            .map_err(|e| format!("Failed to read key file: {}", e))?;
        
        let key_bytes = general_purpose::STANDARD
            .decode(&key_data)
            .map_err(|e| format!("Failed to decode key: {}", e))?;
        
        Ok(key_bytes)
    } else {
        // Generate new key
        let key = generate_machine_key()?;
        
        // Save key (base64 encoded)
        let encoded = general_purpose::STANDARD.encode(&key);
        
        // Create parent directory if needed
        if let Some(parent) = Path::new(key_path).parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create key directory: {}", e))?;
        }
        
        fs::write(key_path, encoded)
            .map_err(|e| format!("Failed to save key: {}", e))?;
        
        // Set restrictive permissions (Unix only)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(key_path)
                .map_err(|e| format!("Failed to get key file metadata: {}", e))?
                .permissions();
            perms.set_mode(0o600); // rw------- (owner only)
            fs::set_permissions(key_path, perms)
                .map_err(|e| format!("Failed to set key file permissions: {}", e))?;
        }
        
        info!("Generated new encryption key");
        Ok(key)
    }
}

/// Encrypt and save API token
pub fn save_token(token: &str) -> Result<(), String> {
    info!("Encrypting and saving API token...");
    
    // Get encryption key
    let key_bytes = get_encryption_key()?;
    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    
    // Generate random nonce
    let nonce_bytes = Aes256Gcm::generate_nonce(&mut OsRng);
    let nonce = Nonce::from_slice(&nonce_bytes);
    
    // Encrypt token
    let ciphertext = cipher
        .encrypt(nonce, token.as_bytes())
        .map_err(|e| format!("Encryption failed: {}", e))?;
    
    // Combine nonce + ciphertext
    let mut encrypted_data = nonce_bytes.to_vec();
    encrypted_data.extend_from_slice(&ciphertext);
    
    // Encode to base64
    let encoded = general_purpose::STANDARD.encode(&encrypted_data);
    
    // Determine token file path
    let token_path = if Path::new(TOKEN_FILE).parent().map(|p| p.exists()).unwrap_or(false) {
        TOKEN_FILE
    } else {
        DEV_TOKEN_FILE
    };
    
    // Create parent directory if needed
    if let Some(parent) = Path::new(token_path).parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create token directory: {}", e))?;
    }
    
    // Save encrypted token
    fs::write(token_path, encoded)
        .map_err(|e| format!("Failed to save token: {}", e))?;
    
    // Set restrictive permissions (Unix only)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(token_path)
            .map_err(|e| format!("Failed to get token file metadata: {}", e))?
            .permissions();
        perms.set_mode(0o600); // rw------- (owner only)
        fs::set_permissions(token_path, perms)
            .map_err(|e| format!("Failed to set token file permissions: {}", e))?;
    }
    
    info!("✓ Token encrypted and saved to: {}", token_path);
    Ok(())
}

/// Load and decrypt API token
pub fn load_token() -> Result<String, String> {
    debug!("Loading encrypted API token...");
    
    // Determine token file path
    let token_path = if Path::new(TOKEN_FILE).exists() {
        TOKEN_FILE
    } else if Path::new(DEV_TOKEN_FILE).exists() {
        DEV_TOKEN_FILE
    } else {
        return Err("Token file not found. Please run: device-agent --register <token>".to_string());
    };
    
    // Read encrypted token
    let encoded = fs::read_to_string(token_path)
        .map_err(|e| format!("Failed to read token file: {}", e))?;
    
    // Decode from base64
    let encrypted_data = general_purpose::STANDARD
        .decode(encoded.trim())
        .map_err(|e| format!("Failed to decode token: {}", e))?;
    
    if encrypted_data.len() < 12 {
        return Err("Invalid token file format".to_string());
    }
    
    // Split nonce and ciphertext
    let (nonce_bytes, ciphertext) = encrypted_data.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);
    
    // Get encryption key
    let key_bytes = get_encryption_key()?;
    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    
    // Decrypt token
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| format!("Decryption failed: {}", e))?;
    
    let token = String::from_utf8(plaintext)
        .map_err(|e| format!("Invalid token format: {}", e))?;
    
    debug!("✓ Token decrypted successfully");
    Ok(token)
}

/// Check if token is registered
pub fn has_token() -> bool {
    Path::new(TOKEN_FILE).exists() || Path::new(DEV_TOKEN_FILE).exists()
}

/// Delete saved token
pub fn delete_token() -> Result<(), String> {
    let token_path = if Path::new(TOKEN_FILE).exists() {
        TOKEN_FILE
    } else {
        DEV_TOKEN_FILE
    };
    
    if Path::new(token_path).exists() {
        fs::remove_file(token_path)
            .map_err(|e| format!("Failed to delete token: {}", e))?;
        info!("✓ Token deleted");
    }
    
    Ok(())
}

pub fn get_token_location() -> &'static str {
    if Path::new(TOKEN_FILE).exists() {
        TOKEN_FILE
    } else if Path::new(DEV_TOKEN_FILE).exists() {
        DEV_TOKEN_FILE
    } else {
        "Not registered"
    }
}