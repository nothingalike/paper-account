use na_paper_account::{
    account::Account,
    market::SimpleMarketDataProvider,
    order::{Order, OrderSide},
    types::{Symbol, Price, Quantity},
};
use rust_decimal::Decimal;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Limit Orders Example");
    
    // Create a new paper trading account with an initial deposit
    let mut account = Account::new(
        "Limit Orders Demo",
        "USD",
        Decimal::from(10000),
    );
    
    // Create a simple market data provider
    let mut market_data = SimpleMarketDataProvider::new();
    
    // Set initial price for TSLA
    let tsla = Symbol::new("TSLA");
    market_data.set_price(tsla.clone(), Price::from_f64(250.00));
    
    println!("Initial TSLA price: $250.00");
    println!("Initial balance: ${}", account.cash_balance);
    
    // Place a limit buy order for TSLA at a lower price
    let buy_price = Price::from_f64(240.00);
    let buy_quantity = Quantity::from_f64(5.0);
    
    let limit_buy_order = Order::limit(
        tsla.clone(),
        OrderSide::Buy,
        buy_quantity,
        buy_price,
    );
    
    let buy_order_id = account.submit_order(limit_buy_order)?;
    println!("\nSubmitted limit buy order for {} shares of TSLA at ${}", 
        buy_quantity, 
        buy_price
    );
    
    // Process orders - this should not execute since the price is higher than our limit
    account.process_open_orders(&market_data)?;
    println!("Current TSLA price (${}) is above our limit buy price (${}) - order not executed", 
        Price::from_f64(250.00),
        buy_price
    );
    
    // Lower the market price to trigger our limit buy
    println!("\nMarket price drops to $235.00");
    market_data.set_price(tsla.clone(), Price::from_f64(235.00));
    
    // Process orders again - this should execute our limit buy
    let executed = account.process_limit_order(&buy_order_id, &market_data)?;
    
    if executed {
        println!("Limit buy order executed at ${}", buy_price);
        
        // Print position information
        if let Some(position) = account.get_position(&tsla) {
            println!("\nPosition: {} shares of {} at ${} per share", 
                position.quantity, 
                position.symbol,
                position.average_price
            );
        }
        
        // Place a limit sell order at a higher price
        let sell_price = Price::from_f64(260.00);
        
        let limit_sell_order = Order::limit(
            tsla.clone(),
            OrderSide::Sell,
            buy_quantity, // Sell the same quantity we bought
            sell_price,
        );
        
        let sell_order_id = account.submit_order(limit_sell_order)?;
        println!("\nSubmitted limit sell order for {} shares of TSLA at ${}", 
            buy_quantity, 
            sell_price
        );
        
        // Raise the market price to trigger our limit sell
        println!("\nMarket price rises to $265.00");
        market_data.set_price(tsla.clone(), Price::from_f64(265.00));
        
        // Process orders again - this should execute our limit sell
        let executed = account.process_limit_order(&sell_order_id, &market_data)?;
        
        if executed {
            println!("Limit sell order executed at ${}", sell_price);
            
            // Get account performance
            let performance = account.performance(&market_data)?;
            println!("\nFinal balance: ${}", performance.cash_balance);
            println!("Realized P&L: ${}", performance.realized_pnl);
        }
    }
    
    Ok(())
}
