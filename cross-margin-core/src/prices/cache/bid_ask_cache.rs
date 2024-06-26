use std::{collections::HashMap, sync::Arc};

use crate::prices::{cross::{CrossPriceEngine, SourceInstrument}, dto::CrossMarginBidAsk};


pub struct CrossMarginBidAskCache {
    prices: HashMap<String, Arc<CrossMarginBidAsk>>,
    base_quote_index: HashMap<String, HashMap<String, Arc<CrossMarginBidAsk>>>,
    quote_base_index: HashMap<String, HashMap<String, Arc<CrossMarginBidAsk>>>,
    cross_ending: CrossPriceEngine,
}

impl CrossMarginBidAskCache {
    pub fn new(
        request_crosses: impl IntoIterator<Item = (String, String)>,
        instruments: Vec<SourceInstrument>,
        cached_prices: Vec<CrossMarginBidAsk>,
    ) -> Self {
        let crosses = CrossPriceEngine::new(request_crosses, instruments);

        let mut prices = HashMap::new();
        let mut base_quote_index = HashMap::new();
        let mut quote_base_index = HashMap::new();

        for bid_ask in cached_prices {
            let bid_ask = Arc::new(bid_ask);
            prices.insert(bid_ask.asset_pair.clone(), bid_ask.clone());

            let base_quote = base_quote_index
                .entry(bid_ask.base.clone())
                .or_insert_with(HashMap::new);
            base_quote.insert(bid_ask.quote.clone(), bid_ask.clone());

            let quote_base = quote_base_index
                .entry(bid_ask.quote.clone())
                .or_insert_with(HashMap::new);
            quote_base.insert(bid_ask.base.clone(), bid_ask.clone());
        }

        return CrossMarginBidAskCache {
            prices,
            base_quote_index,
            quote_base_index,
            cross_ending: crosses,
        };
    }

    pub fn handle_new(&mut self, bid_ask: CrossMarginBidAsk) {
        let bid_ask = Arc::new(bid_ask);
        self.prices
            .insert(bid_ask.asset_pair.clone(), bid_ask.clone());

        let base_quote = self
            .base_quote_index
            .entry(bid_ask.base.clone())
            .or_insert_with(HashMap::new);
        base_quote.insert(bid_ask.quote.clone(), bid_ask.clone());

        let quote_base = self
            .quote_base_index
            .entry(bid_ask.quote.clone())
            .or_insert_with(HashMap::new);
        quote_base.insert(bid_ask.base.clone(), bid_ask.clone());
    }

    pub fn get_by_id(&self, id: &str) -> Option<Arc<CrossMarginBidAsk>> {
        self.prices.get(id).cloned()
    }

    pub fn get_base_quote(&self, base: &str, quote: &str) -> Option<Arc<CrossMarginBidAsk>> {
        self.base_quote_index
            .get(base)
            .and_then(|x| x.get(quote))
            .cloned()
    }

    pub fn get_quote_base(&self, quote: &str, base: &str) -> Option<Arc<CrossMarginBidAsk>> {
        self.quote_base_index
            .get(quote)
            .and_then(|x| x.get(base))
            .cloned()
    }

    pub fn get_price(&self, base: &str, quote: &str) -> Option<Arc<CrossMarginBidAsk>> {
        if base == quote {
            return Some(Arc::new(CrossMarginBidAsk::create_blank(base)));
        }

        let result = self.get_base_quote(base, quote).or_else(|| {
            self.get_quote_base(base, quote)
                .map(|x| Arc::new(x.reverse()))
        });

        if let None = result {
            let cross = self.cross_ending.get_cross(base, quote)?;
            return Some(Arc::new(cross.get_bid_ask()));
        }

        result
    }
}


