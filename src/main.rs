//! HFT Market Data Engine - Binary executable
//!
//! Demonstrates real-time market data processing with latency tracking

use hft_market_data_engine::{
    engine::EventLoop,
    feed::market_data::{Symbol, Price, MarketTick, MarketEvent},
    memory::ring_buffer::RingBuffer,
};
use std::time::Instant;

fn main() {
    println!("🚀 HFT Market Data Engine Starting...\n");

    // Initialize engine
    let mut event_loop = EventLoop::new(1024);
    println!("✓ Event loop initialized (capacity: 1024 events)");

    // Create some symbols
    let symbols = vec![
        Symbol::from_str("AAPL"),
        Symbol::from_str("MSFT"),
        Symbol::from_str("GOOGL"),
        Symbol::from_str("TSLA"),
    ];
    println!("✓ Tracking {} symbols\n", symbols.len());

    // Simulate market data
    println!("📊 Simulating market data feed...");
    let start = Instant::now();

    for i in 0..100_000 {
        let symbol = symbols[i % symbols.len()];

        // Create realistic tick
        let tick = MarketTick {
            symbol,
            sequence: i as u64,
            bid_price: Price::new(100.0 + (i as f64 % 50.0) * 0.01),
            bid_size: 1000,
            ask_price: Price::new(100.01 + (i as f64 % 50.0) * 0.01),
            ask_size: 1000,
            timestamp_ns: start.elapsed().as_nanos() as u64,
        };

        let event = MarketEvent::Quote(tick);
        event_loop.process_event(event, start.elapsed().as_nanos() as u64);
    }

    let elapsed = start.elapsed();
    println!("✓ Processed 100,000 events in {:.2}ms\n", elapsed.as_millis());

    // Print statistics
    let stats = event_loop.stats();
    println!("📈 Statistics:");
    println!("  Events processed: {}", stats.events_processed);
    println!("  Total latency: {} ns", stats.total_latency_ns);
    if stats.events_processed > 0 {
        println!(
            "  Average latency: {:.0} ns",
            stats.total_latency_ns as f64 / stats.events_processed as f64
        );
    }

    // Print order books
    println!("\n📋 Order Books:");
    for symbol in &symbols {
        if let Some(ob) = event_loop.get_orderbook(*symbol) {
            if let Some((bid_price, bid_size)) = ob.best_bid() {
                if let Some((ask_price, ask_size)) = ob.best_ask() {
                    println!(
                        "  {}: bid={:.2}x{} ask={:.2}x{}",
                        String::from_utf8_lossy(&symbol.value),
                        bid_price.to_double(),
                        bid_size,
                        ask_price.to_double(),
                        ask_size
                    );
                }
            }
        }
    }

    println!("\n✅ Engine completed successfully!");
}
