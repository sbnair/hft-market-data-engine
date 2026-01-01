//! Market feed handling and message parsing
//!
//! Responsible for:
//! - Consuming raw bytes from exchange connections
//! - Parsing FIX protocol messages
//! - Normalizing multi-venue feeds
//! - Generating typed MarketEvent objects

pub mod market_data;
pub mod fix_parser;
pub mod exchange;

pub use market_data::{MarketTick, Trade, MarketEvent, Symbol, Price};
pub use fix_parser::FixParser;
pub use exchange::ExchangeType;

/// Result type for feed operations
pub type FeedResult<T> = Result<T, FeedError>;

/// Errors that can occur during feed processing
#[derive(Debug, Clone)]
pub enum FeedError {
    InvalidMessage(String),
    ParseError(String),
    BufferFull,
    Disconnected,
}

impl std::fmt::Display for FeedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FeedError::InvalidMessage(msg) => write!(f, "Invalid message: {}", msg),
            FeedError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            FeedError::BufferFull => write!(f, "Buffer is full"),
            FeedError::Disconnected => write!(f, "Feed disconnected"),
        }
    }
}

impl std::error::Error for FeedError {}
