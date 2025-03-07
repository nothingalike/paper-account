use na_paper_account::{
    account::Account,
    market::SimpleMarketDataProvider,
    types::{Symbol, Price},
};
use rust_decimal::Decimal;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Basic Paper Trading Account Example");
    
    // Create a new paper trading account with an initial deposit of $10,000
    let account = Account::new(
        "My Paper Trading Account",
        "USD",
        Decimal::from(10000),
    );
    
    println!("Created account: {}", account.name);
    println!("Initial balance: ${}", account.cash_balance);
    
    // Create a simple market data provider
    let mut market_data = SimpleMarketDataProvider::new();
    
    // Set some example prices
    let aapl = Symbol::new("AAPL");
    let msft = Symbol::new("MSFT");
    let goog = Symbol::new("GOOG");
    
    market_data.set_price(aapl, Price::from_f64(175.50));
    market_data.set_price(msft, Price::from_f64(350.25));
    market_data.set_price(goog, Price::from_f64(142.80));
    
    // Get account performance metrics
    let performance = account.performance(&market_data)?;
    
    println!("\nAccount Summary:");
    println!("Cash Balance: ${}", performance.cash_balance);
    println!("Equity: ${}", performance.equity);
    println!("ROI: {}%", performance.roi);
    
    Ok(())
}
