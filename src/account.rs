use std::collections::HashMap;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::error::{Result, Error};
use crate::market::MarketDataProvider;
use crate::order::{Order, OrderSide, OrderType, Trade};
use crate::position::Position;
use crate::types::{AccountId, OrderId, Price, Symbol};

/// Represents a paper trading account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    /// Unique identifier for the account
    pub id: AccountId,
    /// Name of the account
    pub name: String,
    /// Base currency of the account
    pub base_currency: String,
    /// Cash balance
    pub cash_balance: Decimal,
    /// Initial deposit
    pub initial_deposit: Decimal,
    /// Map of positions by symbol
    pub positions: HashMap<String, Position>,
    /// Map of open orders by order ID
    pub open_orders: HashMap<String, Order>,
    /// List of closed orders
    pub order_history: Vec<Order>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
}

impl Account {
    /// Create a new paper trading account with an initial deposit
    pub fn new<S: Into<String>>(name: S, base_currency: S, initial_deposit: Decimal) -> Self {
        let now = Utc::now();
        Self {
            id: AccountId::new(),
            name: name.into(),
            base_currency: base_currency.into(),
            cash_balance: initial_deposit,
            initial_deposit,
            positions: HashMap::new(),
            open_orders: HashMap::new(),
            order_history: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Get the total equity value of the account (cash + positions)
    pub fn equity<M: MarketDataProvider>(&self, market_data: &M) -> Result<Decimal> {
        let mut equity = self.cash_balance;

        for (_, position) in &self.positions {
            if !position.is_flat() {
                let quote = market_data.get_quote(&position.symbol)?;
                let position_value = position.market_value(quote.mid());
                equity += position_value;
            }
        }

        Ok(equity)
    }

    /// Get a position by symbol
    pub fn get_position(&self, symbol: &Symbol) -> Option<&Position> {
        self.positions.get(&symbol.0)
    }

    /// Get a mutable position by symbol
    pub fn get_position_mut(&mut self, symbol: &Symbol) -> Option<&mut Position> {
        self.positions.get_mut(&symbol.0)
    }

    /// Get or create a position for a symbol
    pub fn get_or_create_position(&mut self, symbol: Symbol) -> &mut Position {
        if !self.positions.contains_key(&symbol.0) {
            let position = Position::new(symbol.clone());
            self.positions.insert(symbol.0.clone(), position);
        }
        self.positions.get_mut(&symbol.0).unwrap()
    }

    /// Get an order by ID
    pub fn get_order(&self, order_id: &OrderId) -> Option<&Order> {
        self.open_orders.get(&order_id.0.to_string())
    }

    /// Get a mutable order by ID
    pub fn get_order_mut(&mut self, order_id: &OrderId) -> Option<&mut Order> {
        self.open_orders.get_mut(&order_id.0.to_string())
    }

    /// Submit a new order
    pub fn submit_order(&mut self, mut order: Order) -> Result<OrderId> {
        // Validate the order
        self.validate_order(&order)?;

        // Update order status
        order.submit();

        // Store the order
        let order_id = order.id;
        self.open_orders.insert(order_id.0.to_string(), order);
        self.updated_at = Utc::now();

        Ok(order_id)
    }

    /// Cancel an order
    pub fn cancel_order(&mut self, order_id: &OrderId) -> Result<()> {
        let order = self
            .get_order_mut(order_id)
            .ok_or_else(|| Error::OrderNotFound {
                order_id: *order_id,
            })?;

        if order.cancel() {
            // Move to order history
            let order_id_str = order_id.0.to_string();
            if let Some(order) = self.open_orders.remove(&order_id_str) {
                self.order_history.push(order);
            }
            self.updated_at = Utc::now();
        }

        Ok(())
    }

    /// Process a market order execution
    pub fn execute_market_order<M: MarketDataProvider>(
        &mut self,
        order_id: &OrderId,
        market_data: &M,
    ) -> Result<()> {
        let order = self
            .get_order_mut(order_id)
            .ok_or_else(|| Error::OrderNotFound {
                order_id: *order_id,
            })?;

        // Only process market orders
        if order.order_type != OrderType::Market {
            return Ok(());
        }

        // Only process active orders
        if !order.is_active() {
            return Ok(());
        }

        // Get current market price
        let quote = market_data.get_quote(&order.symbol)?;
        let execution_price = match order.side {
            OrderSide::Buy => quote.ask,
            OrderSide::Sell => quote.bid,
        };

        // Execute the order at market price
        self.execute_order_at_price(order_id, execution_price)?;

        Ok(())
    }

    /// Process a limit order against the current market price
    pub fn process_limit_order<M: MarketDataProvider>(
        &mut self,
        order_id: &OrderId,
        market_data: &M,
    ) -> Result<bool> {
        let order = self
            .get_order_mut(order_id)
            .ok_or_else(|| Error::OrderNotFound {
                order_id: *order_id,
            })?;

        // Only process limit orders
        if order.order_type != OrderType::Limit {
            return Ok(false);
        }

        // Only process active orders
        if !order.is_active() {
            return Ok(false);
        }

        // Get current market price
        let quote = market_data.get_quote(&order.symbol)?;

        // Check if limit price is met
        let limit_price = order.limit_price.ok_or_else(|| Error::InvalidOrder {
            reason: "Limit order without limit price".to_string(),
        })?;

        let price_met = match order.side {
            OrderSide::Buy => quote.ask.0 <= limit_price.0,
            OrderSide::Sell => quote.bid.0 >= limit_price.0,
        };

        if price_met {
            // Execute the order at limit price
            self.execute_order_at_price(order_id, limit_price)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Execute an order at a specific price
    pub fn execute_order_at_price(&mut self, order_id: &OrderId, price: Price) -> Result<()> {
        // Get the order
        let order = self
            .get_order(order_id)
            .ok_or_else(|| Error::OrderNotFound {
                order_id: *order_id,
            })?
            .clone();

        // Create a trade for the remaining quantity
        let remaining_qty = order.remaining_quantity();
        if remaining_qty.is_zero() {
            return Ok(());
        }

        let trade = Trade::new(
            order.id,
            order.symbol.clone(),
            order.side,
            remaining_qty,
            price,
        );

        // Update position
        let position = self.get_or_create_position(order.symbol.clone());
        position.update_with_trade(&trade);

        // Update cash balance
        match order.side {
            OrderSide::Buy => {
                let cost = trade.quantity.0 * trade.price.0;
                if self.cash_balance < cost {
                    return Err(Error::InsufficientFunds {
                        required: cost,
                        available: self.cash_balance,
                    });
                }
                self.cash_balance -= cost;
            }
            OrderSide::Sell => {
                let proceeds = trade.quantity.0 * trade.price.0;
                self.cash_balance += proceeds;
            }
        }

        // Update the order with the trade
        let order = self
            .get_order_mut(order_id)
            .ok_or_else(|| Error::OrderNotFound {
                order_id: *order_id,
            })?;

        order.add_trade(trade);

        // If order is filled, move it to history
        if order.is_filled() {
            let order_id_str = order_id.0.to_string();
            if let Some(order) = self.open_orders.remove(&order_id_str) {
                self.order_history.push(order);
            }
        }

        self.updated_at = Utc::now();

        Ok(())
    }

    /// Validate an order before submission
    fn validate_order(&self, order: &Order) -> Result<()> {
        // Check if we have enough cash for buy orders
        if order.side == OrderSide::Buy {
            let estimated_cost = match order.order_type {
                OrderType::Market => {
                    // For market orders, we can't know the exact price,
                    // so we should have enough cash for the full quantity
                    // This is a simplification - in practice, you might want to
                    // estimate based on the last price plus some margin
                    return Ok(());
                }
                OrderType::Limit => {
                    let limit_price = order.limit_price.ok_or_else(|| Error::InvalidOrder {
                        reason: "Limit order without limit price".to_string(),
                    })?;
                    order.quantity.0 * limit_price.0
                }
                OrderType::Stop => {
                    // For stop orders, we should have enough cash for the full quantity
                    // at the stop price
                    let stop_price = order.stop_price.ok_or_else(|| Error::InvalidOrder {
                        reason: "Stop order without stop price".to_string(),
                    })?;
                    order.quantity.0 * stop_price.0
                }
                OrderType::StopLimit => {
                    // For stop-limit orders, we should have enough cash for the full quantity
                    // at the limit price
                    let limit_price = order.limit_price.ok_or_else(|| Error::InvalidOrder {
                        reason: "Stop-limit order without limit price".to_string(),
                    })?;
                    order.quantity.0 * limit_price.0
                }
            };

            if self.cash_balance < estimated_cost {
                return Err(Error::InsufficientFunds {
                    required: estimated_cost,
                    available: self.cash_balance,
                });
            }
        }

        // Check if we have enough position for sell orders
        if order.side == OrderSide::Sell {
            if let Some(position) = self.get_position(&order.symbol) {
                if position.quantity.0 < order.quantity.0 {
                    return Err(Error::InsufficientPosition {
                        symbol: order.symbol.clone(),
                        required: order.quantity.0,
                        available: position.quantity.0,
                    });
                }
            } else {
                return Err(Error::InsufficientPosition {
                    symbol: order.symbol.clone(),
                    required: order.quantity.0,
                    available: Decimal::ZERO,
                });
            }
        }

        Ok(())
    }

    /// Process all open orders against current market data
    pub fn process_open_orders<M: MarketDataProvider>(&mut self, market_data: &M) -> Result<()> {
        // Collect all order IDs to avoid borrowing issues
        let order_ids: Vec<OrderId> = self
            .open_orders
            .values()
            .map(|order| order.id)
            .collect();

        for order_id in order_ids {
            // Get the order type
            let order_type = {
                let order = self.get_order(&order_id).ok_or_else(|| Error::OrderNotFound {
                    order_id,
                })?;
                order.order_type
            };

            // Process based on order type
            match order_type {
                OrderType::Market => {
                    self.execute_market_order(&order_id, market_data)?;
                }
                OrderType::Limit => {
                    self.process_limit_order(&order_id, market_data)?;
                }
                OrderType::Stop => {
                    // TODO: Implement stop order processing
                }
                OrderType::StopLimit => {
                    // TODO: Implement stop-limit order processing
                }
            }
        }

        Ok(())
    }

    /// Get the total realized profit/loss
    pub fn total_realized_pnl(&self) -> Decimal {
        self.positions
            .values()
            .map(|position| position.realized_pnl)
            .sum()
    }

    /// Get the total unrealized profit/loss
    pub fn total_unrealized_pnl<M: MarketDataProvider>(&self, market_data: &M) -> Result<Decimal> {
        let mut total = Decimal::ZERO;

        for (_, position) in &self.positions {
            if !position.is_flat() {
                let quote = market_data.get_quote(&position.symbol)?;
                let unrealized_pnl = position.unrealized_pnl(quote.mid());
                total += unrealized_pnl;
            }
        }

        Ok(total)
    }

    /// Get account performance metrics
    pub fn performance<M: MarketDataProvider>(&self, market_data: &M) -> Result<AccountPerformance> {
        let current_equity = self.equity(market_data)?;
        let unrealized_pnl = self.total_unrealized_pnl(market_data)?;
        let realized_pnl = self.total_realized_pnl();
        let total_pnl = realized_pnl + unrealized_pnl;
        
        let roi = if self.initial_deposit > Decimal::ZERO {
            (total_pnl / self.initial_deposit) * Decimal::from(100)
        } else {
            Decimal::ZERO
        };

        Ok(AccountPerformance {
            initial_deposit: self.initial_deposit,
            cash_balance: self.cash_balance,
            equity: current_equity,
            realized_pnl,
            unrealized_pnl,
            total_pnl,
            roi,
        })
    }
}

/// Account performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountPerformance {
    /// Initial deposit
    pub initial_deposit: Decimal,
    /// Current cash balance
    pub cash_balance: Decimal,
    /// Total equity (cash + positions)
    pub equity: Decimal,
    /// Realized profit/loss
    pub realized_pnl: Decimal,
    /// Unrealized profit/loss
    pub unrealized_pnl: Decimal,
    /// Total profit/loss (realized + unrealized)
    pub total_pnl: Decimal,
    /// Return on investment (%)
    pub roi: Decimal,
}
