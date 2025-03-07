use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Represents a trading symbol (e.g., "AAPL", "BTC/USD")
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Symbol(pub String);

impl Symbol {
    pub fn new<S: Into<String>>(symbol: S) -> Self {
        Symbol(symbol.into().to_uppercase())
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents a quantity of an asset
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Quantity(pub Decimal);

impl Quantity {
    pub fn new(quantity: Decimal) -> Self {
        Quantity(quantity)
    }
    
    pub fn from_f64(quantity: f64) -> Self {
        Quantity(Decimal::from_f64(quantity).unwrap_or_else(|| Decimal::ZERO))
    }
    
    pub fn zero() -> Self {
        Quantity(Decimal::ZERO)
    }
    
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
    
    pub fn is_positive(&self) -> bool {
        self.0.is_sign_positive() && !self.0.is_zero()
    }
    
    pub fn is_negative(&self) -> bool {
        self.0.is_sign_negative()
    }
    
    pub fn abs(&self) -> Self {
        Quantity(self.0.abs())
    }
}

impl fmt::Display for Quantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents a price of an asset
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Price(pub Decimal);

impl Price {
    pub fn new(price: Decimal) -> Self {
        Price(price)
    }
    
    pub fn from_f64(price: f64) -> Self {
        Price(Decimal::from_f64(price).unwrap_or_else(|| Decimal::ZERO))
    }
    
    pub fn zero() -> Self {
        Price(Decimal::ZERO)
    }
}

impl fmt::Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Unique identifier for a trade
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TradeId(pub Uuid);

impl TradeId {
    pub fn new() -> Self {
        TradeId(Uuid::new_v4())
    }
}

impl fmt::Display for TradeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Unique identifier for an order
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OrderId(pub Uuid);

impl OrderId {
    pub fn new() -> Self {
        OrderId(Uuid::new_v4())
    }
}

impl fmt::Display for OrderId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Unique identifier for an account
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AccountId(pub Uuid);

impl AccountId {
    pub fn new() -> Self {
        AccountId(Uuid::new_v4())
    }
}

impl fmt::Display for AccountId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
