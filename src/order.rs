use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::types::{Symbol, Quantity, Price, OrderId, TradeId};

/// Represents the side of an order (buy or sell)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderSide {
    Buy,
    Sell,
}

/// Represents the type of an order
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderType {
    /// Market order (executed at the current market price)
    Market,
    /// Limit order (executed at the specified price or better)
    Limit,
    /// Stop order (becomes a market order when the stop price is reached)
    Stop,
    /// Stop-limit order (becomes a limit order when the stop price is reached)
    StopLimit,
}

/// Represents the status of an order
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderStatus {
    /// Order has been created but not yet submitted
    Created,
    /// Order has been submitted to the market
    Submitted,
    /// Order has been partially filled
    PartiallyFilled,
    /// Order has been completely filled
    Filled,
    /// Order has been canceled
    Canceled,
    /// Order has been rejected
    Rejected,
    /// Order has expired
    Expired,
}

/// Represents a trade execution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trade {
    /// Unique identifier for the trade
    pub id: TradeId,
    /// Order ID that generated this trade
    pub order_id: OrderId,
    /// Symbol being traded
    pub symbol: Symbol,
    /// Side of the trade (buy or sell)
    pub side: OrderSide,
    /// Quantity of the trade
    pub quantity: Quantity,
    /// Price of the trade
    pub price: Price,
    /// Commission paid for the trade
    pub commission: rust_decimal::Decimal,
    /// Timestamp of the trade
    pub timestamp: DateTime<Utc>,
}

impl Trade {
    /// Create a new trade
    pub fn new(
        order_id: OrderId,
        symbol: Symbol,
        side: OrderSide,
        quantity: Quantity,
        price: Price,
        commission: rust_decimal::Decimal,
    ) -> Self {
        Self {
            id: TradeId::new(),
            order_id,
            symbol,
            side,
            quantity,
            price,
            commission,
            timestamp: Utc::now(),
        }
    }
    
    /// Calculate the value of the trade
    pub fn value(&self) -> Price {
        Price(self.price.0 * self.quantity.0)
    }
}

/// Represents an order in the paper trading system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Order {
    /// Unique identifier for the order
    pub id: OrderId,
    /// Symbol being traded
    pub symbol: Symbol,
    /// Side of the order (buy or sell)
    pub side: OrderSide,
    /// Type of the order
    pub order_type: OrderType,
    /// Quantity of the order
    pub quantity: Quantity,
    /// Filled quantity of the order
    pub filled_quantity: Quantity,
    /// Limit price (for limit orders)
    pub limit_price: Option<Price>,
    /// Stop price (for stop and stop-limit orders)
    pub stop_price: Option<Price>,
    /// Status of the order
    pub status: OrderStatus,
    /// Timestamp when the order was created
    pub created_at: DateTime<Utc>,
    /// Timestamp when the order was last updated
    pub updated_at: DateTime<Utc>,
    /// Trades executed against this order
    pub trades: Vec<Trade>,
}

impl Order {
    /// Create a new market order
    pub fn market(symbol: Symbol, side: OrderSide, quantity: Quantity) -> Self {
        let now = Utc::now();
        Self {
            id: OrderId::new(),
            symbol,
            side,
            order_type: OrderType::Market,
            quantity,
            filled_quantity: Quantity::zero(),
            limit_price: None,
            stop_price: None,
            status: OrderStatus::Created,
            created_at: now,
            updated_at: now,
            trades: Vec::new(),
        }
    }
    
    /// Create a new limit order
    pub fn limit(symbol: Symbol, side: OrderSide, quantity: Quantity, price: Price) -> Self {
        let now = Utc::now();
        Self {
            id: OrderId::new(),
            symbol,
            side,
            order_type: OrderType::Limit,
            quantity,
            filled_quantity: Quantity::zero(),
            limit_price: Some(price),
            stop_price: None,
            status: OrderStatus::Created,
            created_at: now,
            updated_at: now,
            trades: Vec::new(),
        }
    }
    
    /// Create a new stop order
    pub fn stop(symbol: Symbol, side: OrderSide, quantity: Quantity, stop_price: Price) -> Self {
        let now = Utc::now();
        Self {
            id: OrderId::new(),
            symbol,
            side,
            order_type: OrderType::Stop,
            quantity,
            filled_quantity: Quantity::zero(),
            limit_price: None,
            stop_price: Some(stop_price),
            status: OrderStatus::Created,
            created_at: now,
            updated_at: now,
            trades: Vec::new(),
        }
    }
    
    /// Create a new stop-limit order
    pub fn stop_limit(
        symbol: Symbol,
        side: OrderSide,
        quantity: Quantity,
        stop_price: Price,
        limit_price: Price,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: OrderId::new(),
            symbol,
            side,
            order_type: OrderType::StopLimit,
            quantity,
            filled_quantity: Quantity::zero(),
            limit_price: Some(limit_price),
            stop_price: Some(stop_price),
            status: OrderStatus::Created,
            created_at: now,
            updated_at: now,
            trades: Vec::new(),
        }
    }
    
    /// Check if the order is active
    pub fn is_active(&self) -> bool {
        matches!(
            self.status,
            OrderStatus::Created | OrderStatus::Submitted | OrderStatus::PartiallyFilled
        )
    }
    
    /// Check if the order is completely filled
    pub fn is_filled(&self) -> bool {
        self.status == OrderStatus::Filled
    }
    
    /// Check if the order is canceled
    pub fn is_canceled(&self) -> bool {
        self.status == OrderStatus::Canceled
    }
    
    /// Check if the order is rejected
    pub fn is_rejected(&self) -> bool {
        self.status == OrderStatus::Rejected
    }
    
    /// Check if the order is expired
    pub fn is_expired(&self) -> bool {
        self.status == OrderStatus::Expired
    }
    
    /// Get the remaining quantity to be filled
    pub fn remaining_quantity(&self) -> Quantity {
        Quantity(self.quantity.0 - self.filled_quantity.0)
    }
    
    /// Add a trade to the order
    pub fn add_trade(&mut self, trade: Trade) {
        // Update filled quantity
        self.filled_quantity = Quantity(self.filled_quantity.0 + trade.quantity.0);
        
        // Update status
        if self.filled_quantity.0 >= self.quantity.0 {
            self.status = OrderStatus::Filled;
        } else if self.filled_quantity.0 > rust_decimal::Decimal::ZERO {
            self.status = OrderStatus::PartiallyFilled;
        }
        
        // Add the trade to the list
        self.trades.push(trade);
        
        // Update timestamp
        self.updated_at = Utc::now();
    }
    
    /// Execute the order with a trade
    pub fn execute(&mut self, trade: Trade) {
        self.add_trade(trade);
    }
    
    /// Check if the order is complete (filled, canceled, rejected, or expired)
    pub fn is_complete(&self) -> bool {
        self.is_filled() || self.is_canceled() || self.is_rejected() || self.is_expired()
    }
    
    /// Cancel the order
    pub fn cancel(&mut self) -> bool {
        if self.is_active() {
            self.status = OrderStatus::Canceled;
            self.updated_at = Utc::now();
            true
        } else {
            false
        }
    }
    
    /// Reject the order
    pub fn reject(&mut self, _reason: &str) {
        if self.status == OrderStatus::Created || self.status == OrderStatus::Submitted {
            self.status = OrderStatus::Rejected;
            self.updated_at = Utc::now();
        }
    }
    
    /// Submit the order
    pub fn submit(&mut self) {
        if self.status == OrderStatus::Created {
            self.status = OrderStatus::Submitted;
            self.updated_at = Utc::now();
        }
    }
}
