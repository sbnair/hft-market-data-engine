//! Memory management and allocation-free data structures

pub mod ring_buffer;
pub mod pool;

pub use ring_buffer::RingBuffer;
pub use pool::ObjectPool;

/// Memory statistics
#[derive(Debug, Clone, Default)]
pub struct MemoryStats {
    pub total_allocations: u64,
    pub total_deallocations: u64,
    pub peak_memory: usize,
    pub current_memory: usize,
}

impl MemoryStats {
    pub fn new() -> Self {
        MemoryStats::default()
    }
}
