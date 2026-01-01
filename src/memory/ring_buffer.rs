use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub struct RingBuffer<T: Copy> {
    buffer: Vec<T>,
    capacity: usize,
    write_pos: Arc<AtomicUsize>,
    read_pos: Arc<AtomicUsize>,
}

impl<T: Copy> RingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        RingBuffer {
            buffer: vec![unsafe { std::mem::zeroed() }; capacity],
            capacity,
            write_pos: Arc::new(AtomicUsize::new(0)),
            read_pos: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn enqueue(&self, item: T) -> Result<(), T> {
        let write = self.write_pos.load(Ordering::Relaxed);
        let next_write = (write + 1) % self.capacity;
        let read = self.read_pos.load(Ordering::Acquire);

        if next_write == read {
            return Err(item); // Buffer full
        }

        unsafe {
            std::ptr::write(&mut self.buffer[write], item);
        }

        self.write_pos.store(next_write, Ordering::Release);
        Ok(())
    }

    pub fn dequeue(&self) -> Option<T> {
        let read = self.read_pos.load(Ordering::Relaxed);
        let write = self.write_pos.load(Ordering::Acquire);

        if read == write {
            return None; // Buffer empty
        }

        let item = self.buffer[read];
        let next_read = (read + 1) % self.capacity;
        self.read_pos.store(next_read, Ordering::Release);

        Some(item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ring_buffer_basic() {
        let rb: RingBuffer<i32> = RingBuffer::new(256);
        
        assert!(rb.enqueue(42).is_ok());
        assert_eq!(rb.dequeue(), Some(42));
        assert_eq!(rb.dequeue(), None);
    }

    #[test]
    fn test_ring_buffer_full() {
        let rb: RingBuffer<u8> = RingBuffer::new(2);
        
        assert!(rb.enqueue(1).is_ok());
        assert!(rb.enqueue(2).is_ok());
        assert!(rb.enqueue(3).is_err());
    }
}
