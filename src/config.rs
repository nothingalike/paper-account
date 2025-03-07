use dotenv::dotenv;
use rust_decimal::Decimal;
use std::env;
use std::str::FromStr;

/// Configuration for the paper trading account
#[derive(Debug, Clone)]
pub struct Config {
    /// Default slippage rate for market orders (as a decimal, e.g., 0.001 for 0.1%)
    pub default_slippage: Decimal,
    /// Default spread between bid and ask prices (as a decimal, e.g., 0.0005 for 0.05%)
    pub default_spread: Decimal,
    /// Commission rate for trades (as a decimal, e.g., 0.0025 for 0.25%)
    pub commission_rate: Decimal,
    /// Log level for the library
    pub log_level: String,
    /// Path for data persistence (if enabled)
    pub storage_path: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_slippage: Decimal::ZERO,
            default_spread: Decimal::ZERO,
            commission_rate: Decimal::ZERO,
            log_level: "info".to_string(),
            storage_path: None,
        }
    }
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        // Load .env file if it exists
        let _ = dotenv();
        
        Self {
            default_slippage: get_decimal_env("PAPER_ACCOUNT_DEFAULT_SLIPPAGE", Decimal::ZERO),
            default_spread: get_decimal_env("PAPER_ACCOUNT_DEFAULT_SPREAD", Decimal::ZERO),
            commission_rate: get_decimal_env("PAPER_ACCOUNT_COMMISSION_RATE", Decimal::ZERO),
            log_level: env::var("PAPER_ACCOUNT_LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
            storage_path: env::var("PAPER_ACCOUNT_STORAGE_PATH").ok(),
        }
    }
}

/// Global configuration instance
static mut CONFIG: Option<Config> = None;

/// Initialize the global configuration
pub fn init() {
    unsafe {
        CONFIG = Some(Config::from_env());
    }
}

/// Get the global configuration
pub fn get() -> Config {
    unsafe {
        match &CONFIG {
            Some(config) => config.clone(),
            None => {
                let config = Config::from_env();
                CONFIG = Some(config.clone());
                config
            }
        }
    }
}

/// Helper function to get a decimal value from an environment variable
fn get_decimal_env(key: &str, default: Decimal) -> Decimal {
    match env::var(key) {
        Ok(val) => Decimal::from_str(&val).unwrap_or(default),
        Err(_) => default,
    }
}
