//! Exchange type definitions and configurations

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExchangeType {
    Nasdaq,
    Nyse,
    Cboe,
    Eurex,
    Other,
}

impl ExchangeType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ExchangeType::Nasdaq => "NASDAQ",
            ExchangeType::Nyse => "NYSE",
            ExchangeType::Cboe => "CBOE",
            ExchangeType::Eurex => "EUREX",
            ExchangeType::Other => "OTHER",
        }
    }

    /// Protocol used by this exchange
    pub fn protocol(&self) -> &'static str {
        match self {
            ExchangeType::Nasdaq => "ITCH",
            ExchangeType::Nyse => "TAQ",
            ExchangeType::Cboe => "PITCH",
            ExchangeType::Eurex => "FIX",
            ExchangeType::Other => "UNKNOWN",
        }
    }

    /// Message rate (typical messages per second)
    pub fn typical_msg_rate(&self) -> u64 {
        match self {
            ExchangeType::Nasdaq => 1_000_000,
            ExchangeType::Nyse => 500_000,
            ExchangeType::Cboe => 250_000,
            ExchangeType::Eurex => 100_000,
            ExchangeType::Other => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exchange_type_protocols() {
        assert_eq!(ExchangeType::Nasdaq.protocol(), "ITCH");
        assert_eq!(ExchangeType::Cboe.protocol(), "PITCH");
    }
}
