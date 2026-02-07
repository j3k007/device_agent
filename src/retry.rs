use backoff::{ExponentialBackoff, Error as BackoffError};
use log::{warn, debug};
use std::time::Duration;

/// Retry configuration
pub struct RetryConfig {
    pub max_retries: u32,
    pub initial_delay_ms: u64,
    pub max_delay_ms: u64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        RetryConfig {
            max_retries: 5,
            initial_delay_ms: 1000,  // 1 second
            max_delay_ms: 60000,     // 60 seconds
        }
    }
}

/// Retry a fallible operation with exponential backoff
pub fn retry_with_backoff<F, T, E>(
    operation_name: &str,
    config: &RetryConfig,
    mut operation: F,
) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
    E: std::fmt::Display,
{
    let backoff = ExponentialBackoff {
        initial_interval: Duration::from_millis(config.initial_delay_ms),
        max_interval: Duration::from_millis(config.max_delay_ms),
        max_elapsed_time: None,
        ..Default::default()
    };

    let mut attempt = 0;
    let result = backoff::retry(backoff, || {
        attempt += 1;
        
        debug!("Attempt #{} for: {}", attempt, operation_name);
        
        match operation() {
            Ok(value) => {
                if attempt > 1 {
                    debug!("Operation '{}' succeeded on attempt #{}", operation_name, attempt);
                }
                Ok(value)
            }
            Err(e) => {
                if attempt < config.max_retries {
                    warn!(
                        "Operation '{}' failed on attempt #{}/{}: {}. Retrying...",
                        operation_name, attempt, config.max_retries, e
                    );
                    Err(BackoffError::transient(e))
                } else {
                    warn!(
                        "Operation '{}' failed after {} attempts: {}",
                        operation_name, attempt, e
                    );
                    Err(BackoffError::permanent(e))
                }
            }
        }
    });

    match result {
        Ok(value) => Ok(value),
        Err(BackoffError::Permanent(e)) => Err(e),
        Err(BackoffError::Transient { err, .. }) => Err(err),
    }
}