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
static INIT: std::sync::Once = std::sync::Once::new();

/// Initialize the library
pub fn init() {
    INIT.call_once(|| {
        config::init();
    });
}
