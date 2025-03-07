//! # Paper Account
//! 
//! `paper_account` is a Rust library for simulating trading accounts without using real money.
//! It provides functionality for managing a paper trading account, including:
//! 
//! - Account creation and management
//! - Order placement (market, limit)
//! - Position tracking
//! - Portfolio valuation
//! - Trade history

extern crate log;
extern crate env_logger;

use std::sync::Once;
use log::{debug, info, LevelFilter};

pub mod account;
pub mod order;
pub mod position;
pub mod error;
pub mod types;
pub mod market;
pub mod config;
pub mod manager;

// Re-export commonly used types
pub use account::Account;
pub use order::{Order, OrderType, OrderSide, OrderStatus};
pub use position::Position;
pub use error::Error;
pub use types::{Symbol, Quantity, Price, TradeId, OrderId, AccountId};
pub use config::Config;
pub use manager::AccountManager;

// Initialize configuration when the library is loaded
#[allow(unused_variables)]
static INIT: Once = Once::new();
static LOGGER_INIT: Once = Once::new();

/// Initialize the library with default configuration
pub fn init() {
    INIT.call_once(|| {
        debug!("Library initialization with default config");
        config::init();
    });
}

/// Initialize the library with custom configuration
pub fn init_with_config(config: Config) {
    INIT.call_once(|| {
        info!("Library initialization with custom config");
        config::init_with_config(config);
    });
}

/// Initialize the logger with a specific log level
/// 
/// This function initializes the env_logger with the specified log level.
/// If not called, logging will not be enabled.
/// 
/// # Examples
/// 
/// ```
/// use na_paper_account::init_logger;
/// 
/// // Initialize with debug level
/// init_logger("debug");
/// ```
pub fn init_logger(level: &str) {
    LOGGER_INIT.call_once(|| {
        // Convert string level to LevelFilter
        let level_filter = match level.to_lowercase().as_str() {
            "trace" => LevelFilter::Trace,
            "debug" => LevelFilter::Debug,
            "info" => LevelFilter::Info,
            "warn" => LevelFilter::Warn,
            "error" => LevelFilter::Error,
            _ => {
                eprintln!("Invalid log level: '{}', defaulting to 'info'", level);
                LevelFilter::Info
            }
        };
        
        let env = env_logger::Env::default()
            .filter_or("RUST_LOG", format!("na_paper_account={}", level));
        
        env_logger::Builder::from_env(env)
            .format_timestamp_millis()
            .filter_module("na_paper_account", level_filter)
            .init();
        
        info!("Logger initialized with level: {}", level);
    });
}
