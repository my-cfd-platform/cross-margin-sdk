use service_sdk::rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::prices::dto::CrossMarginBidAsk;

#[derive(Debug, Clone)]
pub struct SourceInstrument {
    pub id: String,
    pub base: String,
    pub quote: String,
    pub active_price: CrossMarginBidAsk,
}

pub trait CrossCalculationType {
    fn calculate_cross(&self) -> (f64, f64);
}

#[derive(Clone, Debug)]
pub enum CrossPairType {
    SameSide(CrossPairsSameSide),
    DiffSide(CrossPairsDiffSide),
}

impl CrossPairType {
    pub fn handle_price(&mut self, price: CrossMarginBidAsk) {
        match self {
            CrossPairType::SameSide(x) => x.handle_price(price),
            CrossPairType::DiffSide(x) => {
                x.left = BidAskReverseType::Direct(price.clone());
                x.right = BidAskReverseType::Direct(price.clone());
            }
        }
    }

    pub fn calculate_cross(&self) -> (f64, f64) {
        match self {
            CrossPairType::SameSide(x) => x.calculate_cross(),
            CrossPairType::DiffSide(x) => x.calculate_cross(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CrossPairsSameSide {
    pub left: CrossMarginBidAsk,
    pub right: CrossMarginBidAsk,
}

impl CrossPairsSameSide {
    pub fn handle_price(&mut self, price: CrossMarginBidAsk) {
        if self.left.base != price.base || self.left.quote != price.quote {
            self.left = price.clone();
        }

        if self.right.base != price.base || self.right.quote != price.quote {
            self.right = price.clone();
        }
    }
}

impl CrossCalculationType for CrossPairsSameSide {
    fn calculate_cross(&self) -> (f64, f64) {
        let bid = self.left.bid / self.right.ask;
        let ask = self.left.ask / self.right.bid;

        return (bid, ask);
    }
}

#[derive(Clone, Debug)]
pub struct CrossPairsDiffSide {
    pub left: BidAskReverseType,
    pub right: BidAskReverseType,
}

impl CrossPairsDiffSide {
    pub fn handle_price(&mut self, price: CrossMarginBidAsk) {
        if self.left.get_source().base != price.base && self.left.get_source().quote != price.quote
        {
            self.left = match self.left {
                BidAskReverseType::Direct(_) => BidAskReverseType::Direct(price.clone()),
                BidAskReverseType::Reversed(_) => BidAskReverseType::Reversed(price.clone()),
            };
            return;
        }

        if self.right.get_source().base != price.base
            && self.right.get_source().quote != price.quote
        {
            self.left = match self.left {
                BidAskReverseType::Direct(_) => BidAskReverseType::Direct(price.clone()),
                BidAskReverseType::Reversed(_) => BidAskReverseType::Reversed(price.clone()),
            };
            return;
        }
    }
}

impl CrossCalculationType for CrossPairsDiffSide {
    fn calculate_cross(&self) -> (f64, f64) {
        let left = self.left.get_bid_ask();
        let right = self.right.get_bid_ask();
        let bid = left.bid * right.bid;
        let ask = left.ask * right.ask;

        return (bid, ask);
    }
}
#[derive(Clone, Debug)]
pub enum BidAskReverseType {
    Direct(CrossMarginBidAsk),
    Reversed(CrossMarginBidAsk),
}

impl BidAskReverseType {
    pub fn get_bid_ask(&self) -> CrossMarginBidAsk {
        match self {
            BidAskReverseType::Direct(mt) => mt.clone(),
            BidAskReverseType::Reversed(mt) => CrossMarginBidAsk {
                asset_pair: mt.asset_pair.clone(),
                bid: 1.0 / mt.ask,
                ask: 1.0 / mt.bid,
                base: mt.quote.clone(),
                quote: mt.base.clone(),
                date: mt.date,
            },
        }
    }

    pub fn get_source(&self) -> &CrossMarginBidAsk {
        match self {
            BidAskReverseType::Direct(mt) => mt,
            BidAskReverseType::Reversed(mt) => mt,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CrossInstrument {
    pub id: String,
    pub base: String,
    pub quote: String,
    pub prices: CrossPairType,
}

impl CrossInstrument {
    pub fn handle_price(&mut self, price: CrossMarginBidAsk) {
        self.prices.handle_price(price);
    }

    pub fn get_bid_ask(&self) -> CrossMarginBidAsk {
        let (bid, ask) = self.prices.calculate_cross();
        CrossMarginBidAsk {
            asset_pair: format!("{}{}", self.base, self.quote),
            bid,
            ask,
            base: self.base.clone(),
            quote: self.quote.clone(),
            date: DateTimeAsMicroseconds::from(123456 as i64),
        }
    }
}
