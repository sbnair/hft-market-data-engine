use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Symbol {
    pub value: [u8; 8],
}

impl Symbol {
    pub fn from_str(s: &str) -> Self {
        let mut value = [b' '; 8];
        for (i, byte) in s.as_bytes().iter().enumerate().take(8) {
            value[i] = *byte;
        }
        Symbol { value }
    }

    pub fn as_str(&self) -> &str {
        let len = self.value.iter().position(|&b| b == b' ').unwrap_or(8);
        std::str::from_utf8(&self.value[..len]).unwrap_or("INVALID")
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Price(i32);

impl Price {
    pub fn new(p: f64) -> Self {
        Price((p * 10000.0) as i32)
    }

    pub fn to_double(&self) -> f64 {
        self.0 as f64 / 10000.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MarketTick {
    pub symbol: Symbol,
    pub sequence: u64,
    pub bid_price: Price,
    pub bid_size: i32,
    pub ask_price: Price,
    pub ask_size: i32,
    pub timestamp_ns: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct Trade {
    pub symbol: Symbol,
    pub price: Price,
    pub size: i32,
    pub buyer_order_id: u64,
    pub seller_order_id: u64,
    pub timestamp_ns: u64,
}

#[derive(Debug, Clone, Copy)]
pub enum MarketEvent {
    Quote(MarketTick),
    Trade(Trade),
    OrderAccept { order_id: u64, timestamp_ns: u64 },
}
