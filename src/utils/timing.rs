//! High-resolution timing utilities

use std::time::{Instant, Duration};

/// CPU timestamp counter (x86_64 specific)
#[inline]
pub fn rdtsc() -> u64 {
    #[cfg(target_arch = "x86_64")]
    {
        unsafe {
            let eax: u32;
            let edx: u32;
            asm!(
                "rdtsc",
                out("eax") eax,
                out("edx") edx,
                options(nomem, nostack),
            );
            ((edx as u64) << 32) | (eax as u64)
        }
    }

    #[cfg(not(target_arch = "x86_64"))]
    {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64
    }
}

/// High-resolution timer
pub struct Stopwatch {
    start: Instant,
}

impl Stopwatch {
    pub fn new() -> Self {
        Stopwatch {
            start: Instant::now(),
        }
    }

    pub fn elapsed_ns(&self) -> u64 {
        self.start.elapsed().as_nanos() as u64
    }

    pub fn elapsed_us(&self) -> u64 {
        self.start.elapsed().as_micros() as u64
    }

    pub fn elapsed_ms(&self) -> u64 {
        self.start.elapsed().as_millis() as u64
    }

    pub fn reset(&mut self) {
        self.start = Instant::now();
    }
}

impl Default for Stopwatch {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_stopwatch() {
        let mut sw = Stopwatch::new();
        thread::sleep(Duration::from_millis(10));
        
        let elapsed = sw.elapsed_ms();
        assert!(elapsed >= 10);
        
        sw.reset();
        assert!(sw.elapsed_ms() < 5);
    }
}
