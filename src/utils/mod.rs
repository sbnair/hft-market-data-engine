//! Utility functions and helpers

pub mod timing;

pub use timing::{rdtsc, Stopwatch};

/// Safe division helper
pub fn safe_div(num: f64, denom: f64) -> f64 {
    if denom.abs() < 1e-10 { 0.0 } else { num / denom }
}

/// Milliseconds to nanoseconds
pub fn ms_to_ns(ms: u64) -> u64 {
    ms * 1_000_000
}

/// Microseconds to nanoseconds
pub fn us_to_ns(us: u64) -> u64 {
    us * 1_000
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversions() {
        assert_eq!(ms_to_ns(1), 1_000_000);
        assert_eq!(us_to_ns(1), 1_000);
    }
}
