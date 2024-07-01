use serde::{Deserialize, Serialize};
use service_sdk::rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::CrossMarginPositionSide;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossMarginBidAsk {
    pub asset_pair: String,
    pub bid: f64,
    pub ask: f64,
    pub base: String,
    pub quote: String,
    pub date: DateTimeAsMicroseconds,
}

impl CrossMarginBidAsk {
    pub fn get_open_price(&self, side: &CrossMarginPositionSide) -> f64 {
        match side {
            CrossMarginPositionSide::Buy => self.ask,
            CrossMarginPositionSide::Sell => self.bid,
        }
    }

    pub fn get_close_price(&self, side: &CrossMarginPositionSide) -> f64 {
        match side {
            CrossMarginPositionSide::Buy => self.bid,
            CrossMarginPositionSide::Sell => self.ask,
        }
    }

    pub fn reverse(&self) -> CrossMarginBidAsk {
        CrossMarginBidAsk {
            asset_pair: self.asset_pair.clone(),
            bid: 1.0 / self.bid,
            ask: 1.0 / self.ask,
            base: self.quote.clone(),
            quote: self.base.clone(),
            date: self.date,
        }
    }

    pub fn create_blank(ticker: &str) -> CrossMarginBidAsk {
        CrossMarginBidAsk {
            asset_pair: ticker.to_string(),
            bid: 1.0,
            ask: 1.0,
            base: ticker.to_string(),
            quote: ticker.to_string(),
            date: DateTimeAsMicroseconds::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_open_price_buy() {
        let bid_ask = CrossMarginBidAsk {
            asset_pair: "BTC/USD".to_string(),
            bid: 10000.0,
            ask: 10010.0,
            base: "BTC".to_string(),
            quote: "USD".to_string(),
            date: DateTimeAsMicroseconds::from(1634567890 as i64),
        };
        let side = CrossMarginPositionSide::Buy;
        let open_price = bid_ask.get_open_price(&side);
        assert_eq!(open_price, 10010.0);
    }

    #[test]
    fn test_get_open_price_sell() {
        let bid_ask = CrossMarginBidAsk {
            asset_pair: "BTC/USD".to_string(),
            bid: 10000.0,
            ask: 10010.0,
            base: "BTC".to_string(),
            quote: "USD".to_string(),
            date: DateTimeAsMicroseconds::from(1634567890 as i64),
        };
        let side = CrossMarginPositionSide::Sell;
        let open_price = bid_ask.get_open_price(&side);
        assert_eq!(open_price, 10000.0);
    }

    #[test]
    fn test_get_close_price_buy() {
        let bid_ask = CrossMarginBidAsk {
            asset_pair: "BTC/USD".to_string(),
            bid: 10000.0,
            ask: 10010.0,
            base: "BTC".to_string(),
            quote: "USD".to_string(),
            date: DateTimeAsMicroseconds::from(1634567890 as i64),
        };
        let side = CrossMarginPositionSide::Buy;
        let close_price = bid_ask.get_close_price(&side);
        assert_eq!(close_price, 10000.0);
    }

    #[test]
    fn test_get_close_price_sell() {
        let bid_ask = CrossMarginBidAsk {
            asset_pair: "BTC/USD".to_string(),
            bid: 10000.0,
            ask: 10010.0,
            base: "BTC".to_string(),
            quote: "USD".to_string(),
            date: DateTimeAsMicroseconds::from(1634567890 as i64),
        };
        let side = CrossMarginPositionSide::Sell;
        let close_price = bid_ask.get_close_price(&side);
        assert_eq!(close_price, 10010.0);
    }

    #[test]
    fn test_reverse() {
        let bid_ask = CrossMarginBidAsk {
            asset_pair: "BTC/USD".to_string(),
            bid: 10000.0,
            ask: 10010.0,
            base: "BTC".to_string(),
            quote: "USD".to_string(),
            date: DateTimeAsMicroseconds::from(1634567890 as i64),
        };
        let reversed_bid_ask = bid_ask.reverse();
        assert_eq!(reversed_bid_ask.asset_pair, "BTC/USD");
        assert_eq!(reversed_bid_ask.bid, 0.0001);
        assert_eq!(reversed_bid_ask.base, "USD");
        assert_eq!(reversed_bid_ask.quote, "BTC");
    }

    #[test]
    fn test_create_blank() {
        let ticker = "BTC/USD";
        let blank_bid_ask = CrossMarginBidAsk::create_blank(ticker);
        assert_eq!(blank_bid_ask.asset_pair, "BTC/USD");
        assert_eq!(blank_bid_ask.bid, 1.0);
        assert_eq!(blank_bid_ask.ask, 1.0);
        assert_eq!(blank_bid_ask.base, "BTC/USD");
        assert_eq!(blank_bid_ask.quote, "BTC/USD");
    }
}
