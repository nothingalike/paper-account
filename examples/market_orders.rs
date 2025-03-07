use na_paper_account::{
    account::Account,
    market::SimpleMarketDataProvider,
    order::{Order, OrderSide},
    types::{Symbol, Price, Quantity},
};
use rust_decimal::Decimal;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Market Orders Example");
    
    // Create a new paper trading account with an initial deposit of $10,000
    let mut account = Account::new(
        "Market Orders Demo",
        "USD",
        Decimal::from(10000),
    );
    
    // Create a simple market data provider
    let mut market_data = SimpleMarketDataProvider::new();
    
    // Set some example prices
    let aapl = Symbol::new("AAPL");
    market_data.set_price(aapl.clone(), Price::from_f64(175.50));
    
    // Place a market buy order for AAPL
    let aapl_order = Order::market(
        aapl.clone(),
        OrderSide::Buy,
        Quantity::from_f64(10.0),
    );
    
    println!("Initial balance: ${}", account.cash_balance);
    
    // Submit and execute the order
    let order_id = account.submit_order(aapl_order)?;
    println!("Submitted market buy order for AAPL, order ID: {}", order_id);
    
    account.execute_market_order(&order_id, &market_data)?;
    println!("Executed market buy order");
    
    // Print updated account information
    println!("\nUpdated balance: ${}", account.cash_balance);
    
    // Print position information
    if let Some(position) = account.get_position(&aapl) {
        println!("\nPosition: {} shares of {} at ${} per share", 
            position.quantity, 
            position.symbol,
            position.average_price
        );
    }
    
    // Now place a market sell order for half of our position
    if let Some(position) = account.get_position(&aapl) {
        let half_quantity = Quantity(position.quantity.0 / Decimal::from(2));
        
        let sell_order = Order::market(
            aapl.clone(),
            OrderSide::Sell,
            half_quantity,
        );
        
        let sell_order_id = account.submit_order(sell_order)?;
        println!("\nSubmitted market sell order for {} shares of AAPL, order ID: {}", 
            half_quantity, 
            sell_order_id
        );
        
        account.execute_market_order(&sell_order_id, &market_data)?;
        println!("Executed market sell order");
        
        // Print final account information
        println!("\nFinal balance: ${}", account.cash_balance);
        
        // Get account performance
        let performance = account.performance(&market_data)?;
        println!("Realized P&L: ${}", performance.realized_pnl);
    }
    
    Ok(())
}
