use paper_account::{
    account::Account,
    market::{SimpleMarketDataProvider, MarketDataProvider},
    order::{Order, OrderSide},
    types::{Symbol, Price, Quantity},
};
use rust_decimal::Decimal;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Portfolio Management Example");
    
    // Create a new paper trading account with an initial deposit
    let mut account = Account::new(
        "Portfolio Demo",
        "USD",
        Decimal::from(100000),
    );
    
    // Create a simple market data provider
    let mut market_data = SimpleMarketDataProvider::new();
    
    // Set initial prices for various stocks
    let aapl = Symbol::new("AAPL");
    let msft = Symbol::new("MSFT");
    let amzn = Symbol::new("AMZN");
    let goog = Symbol::new("GOOG");
    
    market_data.set_price(aapl.clone(), Price::from_f64(175.50));
    market_data.set_price(msft.clone(), Price::from_f64(350.25));
    market_data.set_price(amzn.clone(), Price::from_f64(178.75));
    market_data.set_price(goog.clone(), Price::from_f64(142.80));
    
    println!("Initial balance: ${}", account.cash_balance);
    
    // Create a diversified portfolio by buying multiple stocks
    let symbols = vec![
        (aapl.clone(), Price::from_f64(175.50), Quantity::from_f64(50.0)),
        (msft.clone(), Price::from_f64(350.25), Quantity::from_f64(25.0)),
        (amzn.clone(), Price::from_f64(178.75), Quantity::from_f64(40.0)),
        (goog.clone(), Price::from_f64(142.80), Quantity::from_f64(60.0)),
    ];
    
    // Place and execute market orders for each stock
    for (symbol, price, quantity) in symbols {
        let order = Order::market(
            symbol.clone(),
            OrderSide::Buy,
            quantity,
        );
        
        let order_id = account.submit_order(order)?;
        println!("Buying {} shares of {} at ${}", quantity, symbol, price);
        
        account.execute_market_order(&order_id, &market_data)?;
    }
    
    // Print portfolio summary after initial purchases
    print_portfolio_summary(&account, &market_data)?;
    
    // Simulate market movements
    println!("\n--- Simulating market movements ---");
    
    // AAPL goes up 5%
    market_data.set_price(aapl.clone(), Price::from_f64(184.28));
    println!("AAPL price increased to $184.28 (+5%)");
    
    // MSFT goes down 3%
    market_data.set_price(msft.clone(), Price::from_f64(339.74));
    println!("MSFT price decreased to $339.74 (-3%)");
    
    // AMZN goes up 8%
    market_data.set_price(amzn.clone(), Price::from_f64(193.05));
    println!("AMZN price increased to $193.05 (+8%)");
    
    // GOOG goes down 2%
    market_data.set_price(goog.clone(), Price::from_f64(139.94));
    println!("GOOG price decreased to $139.94 (-2%)");
    
    // Print updated portfolio summary
    println!("\n--- Updated Portfolio ---");
    print_portfolio_summary(&account, &market_data)?;
    
    // Rebalance portfolio by selling some winners and buying more of losers
    println!("\n--- Rebalancing Portfolio ---");
    
    // Sell half of AAPL (winner)
    if let Some(position) = account.get_position(&aapl) {
        let half_quantity = Quantity(position.quantity.0 / Decimal::from(2));
        
        let sell_order = Order::market(
            aapl.clone(),
            OrderSide::Sell,
            half_quantity,
        );
        
        let order_id = account.submit_order(sell_order)?;
        println!("Selling {} shares of AAPL", half_quantity);
        
        account.execute_market_order(&order_id, &market_data)?;
    }
    
    // Sell half of AMZN (winner)
    if let Some(position) = account.get_position(&amzn) {
        let half_quantity = Quantity(position.quantity.0 / Decimal::from(2));
        
        let sell_order = Order::market(
            amzn.clone(),
            OrderSide::Sell,
            half_quantity,
        );
        
        let order_id = account.submit_order(sell_order)?;
        println!("Selling {} shares of AMZN", half_quantity);
        
        account.execute_market_order(&order_id, &market_data)?;
    }
    
    // Buy more MSFT (loser)
    let buy_order = Order::market(
        msft.clone(),
        OrderSide::Buy,
        Quantity::from_f64(15.0),
    );
    
    let order_id = account.submit_order(buy_order)?;
    println!("Buying 15 more shares of MSFT");
    
    account.execute_market_order(&order_id, &market_data)?;
    
    // Buy more GOOG (loser)
    let buy_order = Order::market(
        goog.clone(),
        OrderSide::Buy,
        Quantity::from_f64(30.0),
    );
    
    let order_id = account.submit_order(buy_order)?;
    println!("Buying 30 more shares of GOOG");
    
    account.execute_market_order(&order_id, &market_data)?;
    
    // Print final portfolio summary
    println!("\n--- Final Portfolio After Rebalancing ---");
    print_portfolio_summary(&account, &market_data)?;
    
    Ok(())
}

// Helper function to print portfolio summary
fn print_portfolio_summary(account: &Account, market_data: &SimpleMarketDataProvider) -> Result<(), Box<dyn Error>> {
    println!("\nPortfolio Summary:");
    println!("------------------");
    
    let mut total_value = Decimal::ZERO;
    
    // Print each position
    for (symbol, position) in &account.positions {
        if !position.is_flat() {
            let quote = market_data.get_quote(&position.symbol)?;
            let current_price = quote.mid();
            let market_value = position.market_value(current_price);
            let unrealized_pnl = position.unrealized_pnl(current_price);
            
            println!(
                "{}: {} shares at avg ${} | Current: ${} | Value: ${} | Unrealized P&L: ${}",
                symbol,
                position.quantity,
                position.average_price,
                current_price,
                market_value,
                unrealized_pnl
            );
            
            total_value += market_value;
        }
    }
    
    // Print account summary
    let performance = account.performance(market_data)?;
    
    println!("\nCash Balance: ${}", performance.cash_balance);
    println!("Portfolio Value: ${}", total_value);
    println!("Total Equity: ${}", performance.equity);
    println!("Realized P&L: ${}", performance.realized_pnl);
    println!("Unrealized P&L: ${}", performance.unrealized_pnl);
    println!("Total P&L: ${}", performance.total_pnl);
    println!("ROI: {}%", performance.roi);
    
    Ok(())
}
