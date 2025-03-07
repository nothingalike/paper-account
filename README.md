# Paper Account Manager

A Rust library for paper trading accounts that can be integrated into trading bots.

## Features

- **Account Management**: Create and manage paper trading accounts with initial deposits
- **Order Execution**: Place and execute different order types (market, limit, stop, stop-limit)
- **Position Tracking**: Track positions and average entry prices
- **Portfolio Valuation**: Calculate equity, P&L, and ROI
- **Market Simulation**: Simple market data provider for paper trading
- **Multiple Accounts**: Manage multiple paper trading accounts with different configurations
- **Account Persistence**: Save and load accounts to/from JSON files

## Usage

Add this to your project:

```bash
cargo add na-paper-account
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

### Multiple Accounts Example

```rust
use paper_account::{
    AccountManager, Config, OrderSide, Symbol, Price, Quantity, Order,
    market::SimpleMarketDataProvider,
};
use rust_decimal_macros::dec;
use rust_decimal::Decimal;
use std::str::FromStr;

fn main() -> paper_account::error::Result<()> {
    // Initialize the library
    paper_account::init();
    
    // Create an account manager
    let mut manager = AccountManager::new();
    
    // Create different account configurations
    let conservative_config = Config {
        default_slippage: Decimal::from_str("0.001")?,  // 0.1% slippage
        default_spread: Decimal::from_str("0.0005")?,   // 0.05% spread
        commission_rate: Decimal::from_str("0.0025")?,  // 0.25% commission
        ..Config::default()
    };
    
    // Create accounts with different configurations
    let conservative_id = manager.create_account_with_config(
        "Conservative Portfolio", 
        "USD", 
        dec!(50000),
        conservative_config
    )?;
    
    // Save accounts to disk
    manager.save_accounts()?;
    
    // Load accounts from disk
    let loaded_manager = AccountManager::load_accounts()?;
    
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

### Multiple Accounts
```bash
cargo run --example multiple_accounts
```
Shows how to manage multiple paper trading accounts with different configurations.

## Core Components

- **Account**: Manages the paper trading account, including cash balance, positions, and orders
- **Order**: Represents different order types (market, limit, stop, stop-limit)
- **Position**: Tracks positions and calculates P&L
- **Market**: Provides market data for paper trading
- **AccountManager**: Central registry to manage multiple accounts with different configurations
- **Config**: Configuration settings for accounts including slippage, spread, and commission rates

## Configuration

The library can be configured using environment variables or a `.env` file. The following configuration options are available:

| Environment Variable | Description | Default |
|---------------------|-------------|---------|
| `PAPER_ACCOUNT_DEFAULT_SLIPPAGE` | Default slippage rate for market orders (as a decimal) | 0.0 |
| `PAPER_ACCOUNT_DEFAULT_SPREAD` | Default spread between bid and ask prices (as a decimal) | 0.0 |
| `PAPER_ACCOUNT_COMMISSION_RATE` | Commission rate for trades (as a decimal) | 0.0 |
| `PAPER_ACCOUNT_LOG_LEVEL` | Log level for the library | "info" |
| `PAPER_ACCOUNT_STORAGE_PATH` | Path for data persistence (if enabled) | None |

Example `.env` file:
```
PAPER_ACCOUNT_DEFAULT_SLIPPAGE=0.001
PAPER_ACCOUNT_DEFAULT_SPREAD=0.0005
PAPER_ACCOUNT_COMMISSION_RATE=0.0025
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
