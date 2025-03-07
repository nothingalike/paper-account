use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::types::{Symbol, Price};
use crate::error::{Result, Error};

/// Represents a market quote for a symbol
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quote {
    /// Symbol of the asset
    pub symbol: Symbol,
    /// Bid price
    pub bid: Price,
    /// Ask price
    pub ask: Price,
    /// Last traded price
    pub last: Price,
    /// Timestamp of the quote
    pub timestamp: DateTime<Utc>,
}

impl Quote {
    /// Create a new quote
    pub fn new(symbol: Symbol, bid: Price, ask: Price, last: Price) -> Self {
        Self {
            symbol,
            bid,
            ask,
            last,
            timestamp: Utc::now(),
        }
    }
    
    /// Get the mid price
    pub fn mid(&self) -> Price {
        Price((self.bid.0 + self.ask.0) / rust_decimal::Decimal::from(2))
    }
}

/// Trait for market data providers
pub trait MarketDataProvider {
    /// Get the current quote for a symbol
    fn get_quote(&self, symbol: &Symbol) -> Result<Quote>;
    
    /// Check if a symbol is supported
    fn is_symbol_supported(&self, symbol: &Symbol) -> bool;
}

/// Simple in-memory market data provider for paper trading
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleMarketDataProvider {
    quotes: HashMap<String, Quote>,
}

impl SimpleMarketDataProvider {
    /// Create a new simple market data provider
    pub fn new() -> Self {
        Self {
            quotes: HashMap::new(),
        }
    }
    
    /// Set a quote for a symbol
    pub fn set_quote(&mut self, quote: Quote) {
        self.quotes.insert(quote.symbol.0.clone(), quote);
    }
    
    /// Set a price for a symbol (creates a quote with bid/ask spread based on configuration)
    pub fn set_price(&mut self, symbol: Symbol, price: Price) {
        // Get spread from configuration (use global config as default)
        let config = crate::config::get();
        let half_spread = Price(price.0 * config.default_spread / rust_decimal::Decimal::from(2));
        
        // Calculate bid and ask with the spread
        let bid = Price(price.0 - half_spread.0);
        let ask = Price(price.0 + half_spread.0);
        
        let quote = Quote::new(symbol.clone(), bid, ask, price);
        self.quotes.insert(symbol.0, quote);
    }
    
    /// Set a price for a symbol with a specific configuration
    pub fn set_price_with_config(&mut self, symbol: Symbol, price: Price, config: &crate::config::Config) {
        // Get spread from provided configuration
        let half_spread = Price(price.0 * config.default_spread / rust_decimal::Decimal::from(2));
        
        // Calculate bid and ask with the spread
        let bid = Price(price.0 - half_spread.0);
        let ask = Price(price.0 + half_spread.0);
        
        let quote = Quote::new(symbol.clone(), bid, ask, price);
        self.quotes.insert(symbol.0, quote);
    }
}

impl MarketDataProvider for SimpleMarketDataProvider {
    fn get_quote(&self, symbol: &Symbol) -> Result<Quote> {
        self.quotes
            .get(&symbol.0)
            .cloned()
            .ok_or_else(|| Error::SymbolNotFound {
                symbol: symbol.clone(),
            })
    }
    
    fn is_symbol_supported(&self, symbol: &Symbol) -> bool {
        self.quotes.contains_key(&symbol.0)
    }
}

/// Historical market data point
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoricalDataPoint {
    /// Symbol of the asset
    pub symbol: Symbol,
    /// Open price
    pub open: Price,
    /// High price
    pub high: Price,
    /// Low price
    pub low: Price,
    /// Close price
    pub close: Price,
    /// Volume
    pub volume: f64,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Trait for historical data providers
pub trait HistoricalDataProvider {
    /// Get historical data for a symbol
    fn get_historical_data(
        &self,
        symbol: &Symbol,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        interval: &str,
    ) -> Result<Vec<HistoricalDataPoint>>;
}

/// Simple in-memory historical data provider for paper trading
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleHistoricalDataProvider {
    data: HashMap<String, Vec<HistoricalDataPoint>>,
}

impl SimpleHistoricalDataProvider {
    /// Create a new simple historical data provider
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
    
    /// Add historical data for a symbol
    pub fn add_data(&mut self, symbol: &Symbol, data: Vec<HistoricalDataPoint>) {
        self.data.insert(symbol.0.clone(), data);
    }
}

impl HistoricalDataProvider for SimpleHistoricalDataProvider {
    fn get_historical_data(
        &self,
        symbol: &Symbol,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        _interval: &str,
    ) -> Result<Vec<HistoricalDataPoint>> {
        let data = self.data.get(&symbol.0).ok_or_else(|| Error::SymbolNotFound {
            symbol: symbol.clone(),
        })?;
        
        let filtered_data: Vec<HistoricalDataPoint> = data
            .iter()
            .filter(|point| point.timestamp >= start && point.timestamp <= end)
            .cloned()
            .collect();
        
        Ok(filtered_data)
    }
}
