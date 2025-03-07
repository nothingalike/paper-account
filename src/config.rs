use rust_decimal::Decimal;

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

/// Global configuration instance
static mut CONFIG: Option<Config> = None;

/// Initialize the global configuration with default values
pub fn init() {
    unsafe {
        CONFIG = Some(Config::default());
    }
}

/// Initialize the global configuration with custom values
pub fn init_with_config(config: Config) {
    unsafe {
        CONFIG = Some(config);
    }
}

/// Get the global configuration
pub fn get() -> Config {
    unsafe {
        match &CONFIG {
            Some(config) => config.clone(),
            None => {
                let config = Config::default();
                CONFIG = Some(config.clone());
                config
            }
        }
    }
}
