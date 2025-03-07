use rust_decimal::Decimal;
use log::{debug, info};
use std::sync::Once;

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
static CONFIG_INIT: Once = Once::new();

/// Initialize the global configuration with default values
pub fn init() {
    debug!("Config::init() - Initializing with default values");
    CONFIG_INIT.call_once(|| {
        let default_config = Config::default();
        debug!("Config::init() - Default config initialized: {:?}", default_config);
        unsafe {
            CONFIG = Some(default_config);
        }
    });
}

/// Initialize the global configuration with custom values
pub fn init_with_config(config: Config) {
    info!("Config::init_with_config() - Initializing with custom config");
    debug!("Config::init_with_config() - Custom config: {:?}", config);
    CONFIG_INIT.call_once(|| {
        unsafe {
            CONFIG = Some(config);
        }
    });
}

/// Get the global configuration
pub fn get() -> Config {
    unsafe {
        // If CONFIG is not initialized, initialize it with default values
        if CONFIG.is_none() {
            debug!("Config::get() - No config found, creating default");
            let default_config = Config::default();
            CONFIG = Some(default_config.clone());
            debug!("Config::get() - Created default config: {:?}", default_config);
            return default_config;
        }
        
        // Return a clone of the config
        let config = CONFIG.as_ref().unwrap().clone();
        debug!("Config::get() - Returning existing config with storage_path: {:?}", config.storage_path);
        config
    }
}
