//! Market data normalization across multiple venues

use crate::feed::{MarketEvent, Symbol};

/// Normalizes events from multiple exchanges
pub struct Normalizer {
    last_sequence: std::collections::HashMap<Symbol, u64>,
}

impl Normalizer {
    pub fn new() -> Self {
        Normalizer {
            last_sequence: std::collections::HashMap::new(),
        }
    }

    /// Detect out-of-order messages
    pub fn is_out_of_order(&mut self, symbol: Symbol, sequence: u64) -> bool {
        match self.last_sequence.get(&symbol) {
            None => {
                self.last_sequence.insert(symbol, sequence);
                false
            }
            Some(&last_seq) => {
                if sequence <= last_seq {
                    true
                } else {
                    self.last_sequence.insert(symbol, sequence);
                    false
                }
            }
        }
    }

    /// Detect gaps in sequence numbers
    pub fn detect_gap(&self, symbol: Symbol, sequence: u64) -> Option<u64> {
        self.last_sequence.get(&symbol).and_then(|&last_seq| {
            if sequence > last_seq + 1 {
                Some(sequence - last_seq - 1)
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_out_of_order_detection() {
        let mut norm = Normalizer::new();
        let symbol = Symbol::from_str("AAPL");

        assert!(!norm.is_out_of_order(symbol, 1));
        assert!(!norm.is_out_of_order(symbol, 2));
        assert!(norm.is_out_of_order(symbol, 1));  // Out of order
    }
}
