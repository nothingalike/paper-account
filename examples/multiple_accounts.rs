use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::str::FromStr;

use paper_account::{
    AccountManager, Config, OrderSide, Symbol, Price, Quantity, Order,
    market::SimpleMarketDataProvider,
};

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
    
    let aggressive_config = Config {
        default_slippage: Decimal::from_str("0.002")?,  // 0.2% slippage
        default_spread: Decimal::from_str("0.001")?,    // 0.1% spread
        commission_rate: Decimal::from_str("0.001")?,   // 0.1% commission
        ..Config::default()
    };
    
    // Create two accounts with different configurations
    let conservative_id = manager.create_account_with_config(
        "Conservative Portfolio", 
        "USD", 
        dec!(50000),
        conservative_config
    )?;
    
    let aggressive_id = manager.create_account_with_config(
        "Aggressive Trading", 
        "USD", 
        dec!(25000),
        aggressive_config
    )?;
    
    println!("Created accounts:");
    println!("  Conservative: {}", conservative_id.0);
    println!("  Aggressive: {}", aggressive_id.0);
    
    // Create a market data provider
    let mut market = SimpleMarketDataProvider::new();
    
    // Set prices for some symbols
    let aapl = Symbol("AAPL".to_string());
    let msft = Symbol("MSFT".to_string());
    let googl = Symbol("GOOGL".to_string());
    
    market.set_price(aapl.clone(), Price(dec!(150)));
    market.set_price(msft.clone(), Price(dec!(300)));
    market.set_price(googl.clone(), Price(dec!(2000)));
    
    // Place orders on both accounts
    // Conservative account - buy AAPL
    {
        let conservative_account = manager.get_account_mut(&conservative_id).unwrap();
        let order = Order::market(
            aapl.clone(), 
            OrderSide::Buy, 
            Quantity(dec!(100))
        );
        let order_id = conservative_account.submit_order(order)?;
        conservative_account.execute_market_order(&order_id, &market)?;
    }
    
    // Aggressive account - buy GOOGL
    {
        let aggressive_account = manager.get_account_mut(&aggressive_id).unwrap();
        let order = Order::market(
            googl.clone(), 
            OrderSide::Buy, 
            Quantity(dec!(5))
        );
        let order_id = aggressive_account.submit_order(order)?;
        aggressive_account.execute_market_order(&order_id, &market)?;
    }
    
    // Print account balances and positions
    println!("\nAccount Balances and Positions:");
    
    let conservative_account = manager.get_account(&conservative_id).unwrap();
    println!("Conservative Account:");
    println!("  Cash Balance: ${:.2}", conservative_account.cash_balance);
    println!("  Positions:");
    for (symbol, position) in &conservative_account.positions {
        println!("    {}: {} shares @ ${:.2}", symbol, position.quantity.0, position.average_price.0);
    }
    
    let aggressive_account = manager.get_account(&aggressive_id).unwrap();
    println!("Aggressive Account:");
    println!("  Cash Balance: ${:.2}", aggressive_account.cash_balance);
    println!("  Positions:");
    for (symbol, position) in &aggressive_account.positions {
        println!("    {}: {} shares @ ${:.2}", symbol, position.quantity.0, position.average_price.0);
    }
    
    // Transfer funds between accounts
    println!("\nTransferring $5,000 from Conservative to Aggressive account");
    manager.transfer(&conservative_id, &aggressive_id, dec!(5000))?;
    
    // Print updated balances
    println!("Updated Cash Balances:");
    println!("  Conservative: ${:.2}", manager.get_account(&conservative_id).unwrap().cash_balance);
    println!("  Aggressive: ${:.2}", manager.get_account(&aggressive_id).unwrap().cash_balance);
    
    // Save accounts to storage
    manager.save()?;
    println!("\nAccounts saved to storage");
    
    // Example of loading accounts
    println!("Loading accounts from storage");
    let loaded_manager = AccountManager::load()?;
    println!("Loaded {} accounts", loaded_manager.account_count());
    
    Ok(())
}
