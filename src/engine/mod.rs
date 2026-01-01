//! Event-driven engine for processing market data
//!
//! Responsible for:
//! - Consuming MarketEvent objects
//! - Maintaining order books
//! - Tracking latencies
//! - Emitting normalized market data

pub mod event_loop;
pub mod orderbook;
pub mod normalizer;

pub use event_loop::EventLoop;
pub use orderbook::OrderBook;
pub use normalizer::Normalizer;

/// Engine configuration
#[derive(Debug, Clone)]
pub struct EngineConfig {
    /// Buffer capacity for events
    pub buffer_capacity: usize,
    /// Max number of order books to track
    pub max_orderbooks: usize,
    /// Enable latency tracking
    pub track_latency: bool,
    /// Enable order book snapshots
    pub enable_snapshots: bool,
}

impl Default for EngineConfig {
    fn default() -> Self {
        EngineConfig {
            buffer_capacity: 1 << 20,  // 1M events
            max_orderbooks: 10_000,
            track_latency: true,
            enable_snapshots: true,
        }
    }
}
