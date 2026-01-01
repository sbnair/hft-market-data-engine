use std::collections::BTreeMap;
use crate::feed::market_data::{Symbol, Price};

#[derive(Debug, Clone)]
pub struct Level {
    pub price: Price,
    pub size: i32,
    pub order_count: i32,
}

#[derive(Debug)]
pub struct OrderBook {
    pub symbol: Symbol,
    pub bids: BTreeMap<i32, Level>, // Price -> Level (inverted for descending)
    pub asks: BTreeMap<i32, Level>, // Price -> Level
    pub last_update_ns: u64,
}

impl OrderBook {
    pub fn new(symbol: Symbol) -> Self {
        OrderBook {
            symbol,
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            last_update_ns: 0,
        }
    }

    pub fn update_bid(&mut self, price: Price, size: i32, timestamp_ns: u64) {
        self.last_update_ns = timestamp_ns;
        if size == 0 {
            self.bids.remove(&price.0);
        } else {
            self.bids.insert(price.0, Level {
                price,
                size,
                order_count: 1,
            });
        }
    }

    pub fn update_ask(&mut self, price: Price, size: i32, timestamp_ns: u64) {
        self.last_update_ns = timestamp_ns;
        if size == 0 {
            self.asks.remove(&price.0);
        } else {
            self.asks.insert(price.0, Level {
                price,
                size,
                order_count: 1,
            });
        }
    }

    pub fn best_bid(&self) -> Option<(Price, i32)> {
        self.bids.iter().next_back().map(|(_, level)| (level.price, level.size))
    }

    pub fn best_ask(&self) -> Option<(Price, i32)> {
        self.asks.iter().next().map(|(_, level)| (level.price, level.size))
    }

    pub fn spread(&self) -> Option<i32> {
        let (_, bid_price) = self.best_bid()?;
        let (ask_price, _) = self.best_ask()?;
        Some(ask_price.0 - bid_price.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orderbook_basic() {
        let mut ob = OrderBook::new(Symbol::from_str("AAPL"));
        
        ob.update_bid(Price::new(150.50), 100, 1000);
        ob.update_ask(Price::new(150.51), 100, 1000);

        assert_eq!(ob.best_bid().unwrap().1, 100);
        assert_eq!(ob.best_ask().unwrap().1, 100);
    }
}
