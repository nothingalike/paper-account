use thiserror::Error;
use crate::types::{Symbol, OrderId, AccountId};

/// Error types for the paper account library
#[derive(Error, Debug)]
pub enum Error {
    #[error("Insufficient funds: required {required}, available {available}")]
    InsufficientFunds {
        required: rust_decimal::Decimal,
        available: rust_decimal::Decimal,
    },
    
    #[error("Insufficient position: required {required}, available {available} for {symbol}")]
    InsufficientPosition {
        symbol: Symbol,
        required: rust_decimal::Decimal,
        available: rust_decimal::Decimal,
    },
    
    #[error("Invalid order: {reason}")]
    InvalidOrder {
        reason: String,
    },
    
    #[error("Order not found: {order_id}")]
    OrderNotFound {
        order_id: OrderId,
    },
    
    #[error("Symbol not found: {symbol}")]
    SymbolNotFound {
        symbol: Symbol,
    },
    
    #[error("Account not found: {account_id}")]
    AccountNotFound {
        account_id: AccountId,
    },
    
    #[error("Invalid quantity: {reason}")]
    InvalidQuantity {
        reason: String,
    },
    
    #[error("Invalid price: {reason}")]
    InvalidPrice {
        reason: String,
    },
    
    #[error("Market data error: {reason}")]
    MarketDataError {
        reason: String,
    },
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Decimal error: {0}")]
    DecimalError(#[from] rust_decimal::Error),
    
    #[error("Custom error: {0}")]
    Custom(String),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Result type for the paper account library
pub type Result<T> = std::result::Result<T, Error>;
