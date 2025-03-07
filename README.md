# rs-paper-account

A Rust library for paper trading accounts that can be integrated into trading bots.

## Features

- **Account Management**: Create and manage paper trading accounts with initial deposits
- **Order Execution**: Place and execute different order types (market, limit, stop, stop-limit)
- **Position Tracking**: Track positions and average entry prices
- **Portfolio Valuation**: Calculate equity, P&L, and ROI
- **Market Simulation**: Simple market data provider for paper trading

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
paper_account = { git = "https://github.com/yourusername/rs-paper-account" }
```

## Example

```rust
use paper_account::{
    account::Account,
    market::SimpleMarketDataProvider,
    order::{Order, OrderSide},
    types::{Price, Quantity, Symbol},
};
use rust_decimal::Decimal;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new paper trading account with an initial deposit
    let mut account = Account::new(
        "My Paper Trading Account",
        "USD",
        Decimal::from(10000),
    );
    
    // Create a simple market data provider
    let mut market_data = SimpleMarketDataProvider::new();
    
    // Set some example prices
    let btc = Symbol::new("BTC/USD");
    market_data.set_price(btc.clone(), Price::from_f64(50000.0));
    
    // Place a market buy order
    let order = Order::market(
        btc.clone(),
        OrderSide::Buy,
        Quantity::from_f64(0.1),
    );
    
    // Submit and execute the order
    let order_id = account.submit_order(order)?;
    account.execute_market_order(&order_id, &market_data)?;
    
    // Get account performance
    let performance = account.performance(&market_data)?;
    println!("Equity: ${}", performance.equity);
    
    Ok(())
}
```

## Examples

The repository includes several examples to demonstrate how to use the library:

### Basic Account Setup
```bash
cargo run --example basic_account
```
Demonstrates how to create a paper trading account and set up market data.

### Market Orders
```bash
cargo run --example market_orders
```
Shows how to place and execute market buy and sell orders.

### Limit Orders
```bash
cargo run --example limit_orders
```
Demonstrates how to use limit orders with price triggers.

### Portfolio Management
```bash
cargo run --example portfolio_management
```
Shows how to manage a portfolio with multiple positions, including rebalancing.

### Market Simulation
```bash
cargo run --example market_simulation
```
Demonstrates how to simulate market price movements and process orders against changing prices.

## Core Components

- **Account**: Manages the paper trading account, including cash balance, positions, and orders
- **Order**: Represents different order types (market, limit, stop, stop-limit)
- **Position**: Tracks positions and calculates P&L
- **Market**: Provides market data for paper trading

## License

This project is licensed under the MIT License - see the LICENSE file for details.
