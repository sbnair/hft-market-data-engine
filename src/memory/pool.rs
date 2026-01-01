//! Object pool for allocation-free reuse

use std::sync::Arc;
use parking_lot::Mutex;

/// Pre-allocated object pool to avoid dynamic allocations
pub struct ObjectPool<T> {
    available: Arc<Mutex<Vec<T>>>,
    capacity: usize,
}

impl<T: Clone> ObjectPool<T> {
    /// Create a new pool with pre-allocated objects
    pub fn new(capacity: usize, template: T) -> Self {
        let mut available = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            available.push(template.clone());
        }

        ObjectPool {
            available: Arc::new(Mutex::new(available)),
            capacity,
        }
    }

    /// Get an object from the pool
    pub fn acquire(&self) -> Option<T> {
        let mut available = self.available.lock();
        available.pop()
    }

    /// Return an object to the pool
    pub fn release(&self, obj: T) {
        let mut available = self.available.lock();
        if available.len() < self.capacity {
            available.push(obj);
        }
    }

    /// Current pool utilization
    pub fn utilization(&self) -> f64 {
        let available = self.available.lock();
        (self.capacity - available.len()) as f64 / self.capacity as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_pool() {
        let pool = ObjectPool::new(10, vec![]);
        
        let obj = pool.acquire();
        assert!(obj.is_some());
        
        pool.release(vec![1, 2, 3]);
        assert!(pool.acquire().is_some());
    }
}
