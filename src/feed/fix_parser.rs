use crate::feed::market_data::*;

pub struct FixParser;

impl FixParser {
    /// Parse FIX message (zero-copy on input buffer)
    pub fn parse_quote(msg: &[u8]) -> Result<MarketTick, String> {
        // FIX message format: field=value|field=value|...
        let mut tick = MarketTick {
            symbol: Symbol { value: [0; 8] },
            sequence: 0,
            bid_price: Price(0),
            bid_size: 0,
            ask_price: Price(0),
            ask_size: 0,
            timestamp_ns: 0,
        };

        for field in msg.split(|&b| b == b'|') {
            if field.len() < 3 { continue; }
            
            let (tag, value) = field.split_at(field.iter().position(|&b| b == b'=').unwrap_or(0));
            let value = &value[1..]; // Skip '='

            match tag {
                b"35" if value == b"D" => {} // Quote message
                b"55" => {
                    // Symbol
                    for i in 0..std::cmp::min(value.len(), 8) {
                        tick.symbol.value[i] = value[i];
                    }
                }
                b"132" => tick.sequence = parse_u64(value),
                b"34" => tick.bid_price = Price(parse_i32(value) as i32),
                b"149" => tick.bid_size = parse_i32(value),
                b"35" => tick.ask_price = Price(parse_i32(value) as i32),
                b"150" => tick.ask_size = parse_i32(value),
                b"60" => tick.timestamp_ns = parse_u64(value) * 1_000_000, // Convert ms to ns
                _ => {}
            }
        }

        Ok(tick)
    }
}

fn parse_i32(bytes: &[u8]) -> i32 {
    let mut result = 0i32;
    for &byte in bytes {
        if byte == b'-' { continue; }
        result = result * 10 + (byte - b'0') as i32;
    }
    if bytes.starts_with(b"-") { result = -result; }
    result
}

fn parse_u64(bytes: &[u8]) -> u64 {
    let mut result = 0u64;
    for &byte in bytes {
        result = result * 10 + (byte - b'0') as u64;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_parser() {
        let msg = b"35=D|55=AAPL|132=1000|34=15050|149=100|35=15100|150=100|60=1000";
        let tick = FixParser::parse_quote(msg).unwrap();
        
        assert_eq!(tick.sequence, 1000);
        assert_eq!(tick.bid_price.to_f64(), 150.50);
        assert_eq!(tick.ask_price.to_f64(), 151.00);
    }
}
