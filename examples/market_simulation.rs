use na_paper_account::{
    account::Account,
    market::SimpleMarketDataProvider,
    order::{Order, OrderSide},
    types::{Symbol, Price, Quantity},
};
use rust_decimal::Decimal;
use std::{error::Error, thread, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Market Simulation Example");
    
    // Create a new paper trading account
    let mut account = Account::new(
        "Market Simulation Demo",
        "USD",
        Decimal::from(25000),
    );
    
    // Create a simple market data provider
    let mut market_data = SimpleMarketDataProvider::new();
    
    // Define our trading symbol
    let btc = Symbol::new("BTC/USD");
    
    // Set initial price
    let initial_price = Price::from_f64(50000.0);
    market_data.set_price(btc.clone(), initial_price);
    
    println!("Initial BTC/USD price: ${}", initial_price);
    println!("Initial account balance: ${}", account.cash_balance);
    
    // Place a limit buy order below current price
    let buy_price = Price::from_f64(49500.0);
    let buy_quantity = Quantity::from_f64(0.2);
    
    let limit_buy = Order::limit(
        btc.clone(),
        OrderSide::Buy,
        buy_quantity,
        buy_price,
    );
    
    let _buy_id = account.submit_order(limit_buy)?;
    println!("\nPlaced limit buy order for {} BTC at ${}", buy_quantity, buy_price);
    
    // Simulate market price movements
    println!("\n--- Simulating market price movements ---");
    
    // Define price points for our simulation
    let price_points = vec![
        49800.0, 49600.0, 49400.0, 49200.0, 49000.0, 
        49300.0, 49700.0, 50100.0, 50500.0, 51200.0,
        51500.0, 51300.0, 50800.0, 50200.0, 49900.0,
    ];
    
    let mut sell_order_placed = false;
    
    // Run the simulation
    for (i, price) in price_points.iter().enumerate() {
        // Update the market price
        let current_price = Price::from_f64(*price);
        market_data.set_price(btc.clone(), current_price);
        
        println!("\nTime step {}: BTC/USD price is now ${}", i + 1, current_price);
        
        // Process any open orders against the new price
        account.process_open_orders(&market_data)?;
        
        // Print current positions
        if let Some(position) = account.get_position(&btc) {
            if !position.is_flat() {
                println!("Current position: {} BTC at avg price ${}", 
                    position.quantity, 
                    position.average_price
                );
                
                // Place a limit sell order once we have a position and haven't placed a sell order yet
                if !sell_order_placed && position.quantity.0 > Decimal::ZERO {
                    // Place a limit sell order above current price
                    let sell_price = Price::from_f64(51000.0);
                    let sell_quantity = Quantity::from_f64(0.1);
                    
                    let limit_sell = Order::limit(
                        btc.clone(),
                        OrderSide::Sell,
                        sell_quantity,
                        sell_price,
                    );
                    
                    let _sell_id = account.submit_order(limit_sell)?;
                    println!("Placed limit sell order for {} BTC at ${}", sell_quantity, sell_price);
                    sell_order_placed = true;
                }
            }
        }
        
        // Print current account status
        let performance = account.performance(&market_data)?;
        println!("Cash balance: ${}", performance.cash_balance);
        println!("Total equity: ${}", performance.equity);
        println!("Unrealized P&L: ${}", performance.unrealized_pnl);
        
        // In a real application, we would wait for actual market data
        // Here we just simulate time passing
        thread::sleep(Duration::from_millis(100));
    }
    
    // Final account summary
    println!("\n--- Final Account Summary ---");
    let performance = account.performance(&market_data)?;
    
    println!("Cash Balance: ${}", performance.cash_balance);
    println!("Equity: ${}", performance.equity);
    println!("Realized P&L: ${}", performance.realized_pnl);
    println!("Unrealized P&L: ${}", performance.unrealized_pnl);
    println!("Total P&L: ${}", performance.total_pnl);
    println!("ROI: {}%", performance.roi);
    
    // Print order history
    println!("\n--- Order History ---");
    for (i, order) in account.order_history.iter().enumerate() {
        println!("Order {}: {:?} {:?} {} shares of {} at {}",
            i + 1,
            order.side,
            order.order_type,
            order.quantity,
            order.symbol,
            if let Some(price) = order.limit_price { 
                format!("${}", price) 
            } else { 
                "market price".to_string() 
            }
        );
    }
    
    Ok(())
}
