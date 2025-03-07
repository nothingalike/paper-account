use serde::{Deserialize, Serialize};
use crate::types::{Symbol, Quantity, Price};
use crate::order::{OrderSide, Trade};
use rust_decimal::Decimal;

/// Represents a position in a particular asset
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Position {
    /// Symbol of the asset
    pub symbol: Symbol,
    /// Quantity of the asset
    pub quantity: Quantity,
    /// Average entry price
    pub average_price: Price,
    /// Realized profit/loss
    pub realized_pnl: Decimal,
}

impl Position {
    /// Create a new position
    pub fn new(symbol: Symbol) -> Self {
        Self {
            symbol,
            quantity: Quantity::zero(),
            average_price: Price::zero(),
            realized_pnl: Decimal::ZERO,
        }
    }
    
    /// Update the position with a new trade
    pub fn update_with_trade(&mut self, trade: &Trade) {
        match trade.side {
            OrderSide::Buy => self.add(trade.quantity, trade.price),
            OrderSide::Sell => self.remove(trade.quantity, trade.price),
        }
    }
    
    /// Add to the position
    pub fn add(&mut self, quantity: Quantity, price: Price) {
        if quantity.is_zero() {
            return;
        }
        
        // Calculate new average price
        let current_value = self.quantity.0 * self.average_price.0;
        let new_value = quantity.0 * price.0;
        let new_quantity = self.quantity.0 + quantity.0;
        
        if new_quantity > Decimal::ZERO {
            self.average_price = Price((current_value + new_value) / new_quantity);
        }
        
        self.quantity = Quantity(new_quantity);
    }
    
    /// Remove from the position
    pub fn remove(&mut self, quantity: Quantity, price: Price) {
        if quantity.is_zero() {
            return;
        }
        
        if self.quantity.is_zero() {
            return;
        }
        
        // Calculate realized profit/loss
        let sell_value = quantity.0 * price.0;
        let cost_basis = quantity.0 * self.average_price.0;
        let pnl = sell_value - cost_basis;
        
        self.realized_pnl += pnl;
        
        // Update quantity
        let new_quantity = self.quantity.0 - quantity.0;
        if new_quantity <= Decimal::ZERO {
            // Position is closed
            self.quantity = Quantity::zero();
            // Keep the average price for historical purposes
        } else {
            self.quantity = Quantity(new_quantity);
        }
    }
    
    /// Calculate unrealized profit/loss at current market price
    pub fn unrealized_pnl(&self, current_price: Price) -> Decimal {
        if self.quantity.is_zero() {
            return Decimal::ZERO;
        }
        
        let current_value = self.quantity.0 * current_price.0;
        let cost_basis = self.quantity.0 * self.average_price.0;
        
        current_value - cost_basis
    }
    
    /// Calculate total profit/loss (realized + unrealized)
    pub fn total_pnl(&self, current_price: Price) -> Decimal {
        self.realized_pnl + self.unrealized_pnl(current_price)
    }
    
    /// Get the current market value of the position
    pub fn market_value(&self, current_price: Price) -> Decimal {
        self.quantity.0 * current_price.0
    }
    
    /// Check if the position is long
    pub fn is_long(&self) -> bool {
        self.quantity.is_positive()
    }
    
    /// Check if the position is short
    pub fn is_short(&self) -> bool {
        self.quantity.is_negative()
    }
    
    /// Check if the position is flat (zero)
    pub fn is_flat(&self) -> bool {
        self.quantity.is_zero()
    }
}
