//! Performance metrics and latency tracking

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// Real-time latency metrics
#[derive(Debug, Clone)]
pub struct LatencyMetrics {
    min_ns: Arc<AtomicU64>,
    max_ns: Arc<AtomicU64>,
    sum_ns: Arc<AtomicU64>,
    count: Arc<AtomicU64>,
}

impl LatencyMetrics {
    pub fn new() -> Self {
        LatencyMetrics {
            min_ns: Arc::new(AtomicU64::new(u64::MAX)),
            max_ns: Arc::new(AtomicU64::new(0)),
            sum_ns: Arc::new(AtomicU64::new(0)),
            count: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Record a latency measurement
    pub fn record(&self, latency_ns: u64) {
        // Update min
        loop {
            let current = self.min_ns.load(Ordering::Relaxed);
            if latency_ns >= current {
                break;
            }
            if self.min_ns.compare_exchange_weak(
                current,
                latency_ns,
                Ordering::Release,
                Ordering::Relaxed,
            ).is_ok() {
                break;
            }
        }

        // Update max
        loop {
            let current = self.max_ns.load(Ordering::Relaxed);
            if latency_ns <= current {
                break;
            }
            if self.max_ns.compare_exchange_weak(
                current,
                latency_ns,
                Ordering::Release,
                Ordering::Relaxed,
            ).is_ok() {
                break;
            }
        }

        // Accumulate
        self.sum_ns.fetch_add(latency_ns, Ordering::Release);
        self.count.fetch_add(1, Ordering::Release);
    }

    /// Get average latency
    pub fn average_ns(&self) -> f64 {
        let sum = self.sum_ns.load(Ordering::Acquire);
        let count = self.count.load(Ordering::Acquire);
        if count == 0 { 0.0 } else { sum as f64 / count as f64 }
    }

    /// Get min latency
    pub fn min_ns(&self) -> u64 {
        let val = self.min_ns.load(Ordering::Acquire);
        if val == u64::MAX { 0 } else { val }
    }

    /// Get max latency
    pub fn max_ns(&self) -> u64 {
        self.max_ns.load(Ordering::Acquire)
    }

    /// Get total count
    pub fn count(&self) -> u64 {
        self.count.load(Ordering::Acquire)
    }

    /// Get statistics summary
    pub fn summary(&self) -> String {
        format!(
            "Latency: avg={:.0}ns, min={}ns, max={}ns, samples={}",
            self.average_ns(),
            self.min_ns(),
            self.max_ns(),
            self.count()
        )
    }
}

impl Default for LatencyMetrics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_latency_metrics() {
        let metrics = LatencyMetrics::new();

        metrics.record(1000);
        metrics.record(2000);
        metrics.record(3000);

        assert_eq!(metrics.count(), 3);
        assert_eq!(metrics.min_ns(), 1000);
        assert_eq!(metrics.max_ns(), 3000);
        assert_eq!(metrics.average_ns(), 2000.0);
    }
}
