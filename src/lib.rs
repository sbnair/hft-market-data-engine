//! HFT Market Data Engine - Ultra-low latency market feed processing
//!
//! A production-grade market data handler for high-frequency trading systems.
//! Designed to ingest and normalize multi-venue exchange feeds with sub-microsecond latency.
//!
//! # Features
//!
//! - **FIX Protocol Parsing**: Zero-copy message parsing
//! - **Lock-Free Ring Buffers**: Inter-thread communication without allocations
//! - **Order Book Management**: Real-time bid/ask tracking
//! - **Event-Driven Architecture**: Single-threaded busy-wait event loop
//! - **Performance Metrics**: Built-in latency tracking (p50, p99, p99.9)
//!
//! # Example
//!
//! ```no_run
//! use hft_market_data_engine::engine::EventLoop;
//! use hft_market_data_engine::feed::market_data::Symbol;
//!
//! let mut event_loop = EventLoop::new(1024);
//! let symbol = Symbol::from_str("AAPL");
//! ```

// Module declarations
pub mod feed;
pub mod engine;
pub mod memory;
pub mod metrics;
pub mod utils;

// Re-exports for convenience
pub use feed::market_data::{MarketTick, MarketEvent, Symbol, Price};
pub use engine::EventLoop;
pub use memory::ring_buffer::RingBuffer;
pub use metrics::LatencyMetrics;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_imports() {
        // Verify all modules are accessible
        let _symbol = Symbol::from_str("TEST");
        let _price = Price::new(100.50);
    }
}
