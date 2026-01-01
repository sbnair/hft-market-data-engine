use std::collections::HashMap;
use crate::feed::market_data::*;
use crate::engine::orderbook::OrderBook;
use crate::memory::ring_buffer::RingBuffer;

pub struct EventLoop {
    orderbooks: HashMap<Symbol, OrderBook>,
    event_buffer: RingBuffer<MarketEvent>,
    stats: LoopStats,
}

#[derive(Debug)]
pub struct LoopStats {
    events_processed: u64,
    total_latency_ns: u64,
}

impl EventLoop {
    pub fn new(capacity: usize) -> Self {
        EventLoop {
            orderbooks: HashMap::new(),
            event_buffer: RingBuffer::new(capacity),
            stats: LoopStats {
                events_processed: 0,
                total_latency_ns: 0,
            },
        }
    }

    pub fn process_event(&mut self, event: MarketEvent, now_ns: u64) {
        match event {
            MarketEvent::Quote(tick) => {
                let ob = self.orderbooks.entry(tick.symbol)
                    .or_insert_with(|| OrderBook::new(tick.symbol));

                ob.update_bid(tick.bid_price, tick.bid_size, tick.timestamp_ns);
                ob.update_ask(tick.ask_price, tick.ask_size, tick.timestamp_ns);

                // Latency tracking
                let latency = now_ns.saturating_sub(tick.timestamp_ns);
                self.stats.total_latency_ns += latency;
                self.stats.events_processed += 1;
            }
            _ => {}
        }
    }

    pub fn run(&mut self) {
        // Busy-wait event loop (production-grade)
        loop {
            // Try to dequeue next event
            if let Some(event) = self.event_buffer.dequeue() {
                let now = unsafe { rdtsc() }; // Get CPU timestamp
                self.process_event(event, now);
            }

            // Typically would check for shutdown, metrics reporting, etc.
        }
    }

    pub fn get_orderbook(&self, symbol: Symbol) -> Option<&OrderBook> {
        self.orderbooks.get(&symbol)
    }

    pub fn stats(&self) -> &LoopStats {
        &self.stats
    }
}

// CPU timestamp counter (platform-specific)
#[inline]
unsafe fn rdtsc() -> u64 {
    #[cfg(target_arch = "x86_64")]
    {
        let eax: u32;
        let edx: u32;
        asm!(
            "rdtsc",
            out("eax") eax,
            out("edx") edx,
        );
        ((edx as u64) << 32) | (eax as u64)
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64
    }
}
