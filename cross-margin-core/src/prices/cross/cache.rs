use std::collections::HashMap;

use crate::prices::dto::CrossMarginBidAsk;

use super::dto::{
    BidAskReverseType, CrossInstrument, CrossPairType, CrossPairsDiffSide, CrossPairsSameSide,
    SourceInstrument,
};

pub struct CrossPriceEngine {
    //ID - [ENTITY]
    cross_matrix: HashMap<String, CrossInstrument>,
    //EURUSD - ID
    mapping: HashMap<String, String>,
    //SOURCE-ID - ID
    subscribe: HashMap<String, Vec<String>>,
}

impl CrossPriceEngine {
    pub fn new(request_crosses: impl IntoIterator<Item = (String, String)>, instruments: Vec<SourceInstrument>) -> Self {
        let mut result = vec![];
        let mut mapping = HashMap::new();
        let mut subscribe = HashMap::new();

        for (base, quote) in request_crosses {
            let (left, right) = Self::find_pair(&base, &quote, &instruments)
                .expect(format!("Can't find pair for {}{}", base, quote).as_str());

            let first_instrument = instruments
                .iter()
                .find(|x| x.base == left.base && x.quote == left.quote)
                .unwrap();
            let second_instrument = instruments
                .iter()
                .find(|x| x.base == right.base && x.quote == right.quote)
                .unwrap();

            let (left, right) =
                match first_instrument.base == base || first_instrument.quote == base {
                    true => (first_instrument.clone(), second_instrument.clone()),
                    false => (second_instrument.clone(), first_instrument.clone()),
                };

            let side = match first_instrument.base == second_instrument.base
                || first_instrument.quote == second_instrument.quote
            {
                true => CrossPairType::SameSide(CrossPairsSameSide {
                    left: left.active_price,
                    right: right.active_price,
                }),
                false => CrossPairType::DiffSide(CrossPairsDiffSide {
                    right: if left.quote == right.base {
                        BidAskReverseType::Direct(right.active_price)
                    } else {
                        BidAskReverseType::Reversed(right.active_price)
                    },
                    left: BidAskReverseType::Direct(left.active_price),
                }),
            };

            let id = format!("{}{}", base, quote);

            subscribe
                .entry(first_instrument.id.clone())
                .or_insert(vec![])
                .push(id.clone());

            subscribe
                .entry(first_instrument.id.clone())
                .or_insert(vec![])
                .push(id.clone());

            result.push(CrossInstrument {
                id: id.clone(),
                base: base.to_string(),
                quote: quote.to_string(),
                prices: side,
            });
        }

        for item in &result {
            mapping.insert(format!("{}-{}", item.base, item.quote), item.id.clone());
        }

        return Self {
            cross_matrix: result.into_iter().map(|x| (x.id.clone(), x)).collect(),
            mapping,
            subscribe,
        };
    }

    pub fn handle_bid_ask(&mut self, new_price: CrossMarginBidAsk) {
        let mapping = self.subscribe.get(&new_price.asset_pair);

        if let Some(mapping) = &mapping {
            for map in mapping.iter() {
                let instrument = self.cross_matrix.get_mut(map).unwrap();
                instrument.handle_price(new_price.clone());
            }
        }
    }

    pub fn get_cross(&self, base: &str, quote: &str) -> Option<&CrossInstrument> {
        let id = format!("{}-{}", base, quote);
        let id = self.mapping.get(&id)?;

        self.cross_matrix.get(id.as_str())
    }

    fn find_pair(
        base: &str,
        quote: &str,
        src: &[SourceInstrument],
    ) -> Option<(SourceInstrument, SourceInstrument)> {
        let base_contains_instruments = src.iter().filter(|x| x.base == base || x.quote == base);

        let quote_contains_instruments = src
            .into_iter()
            .filter(|x| x.base == quote || x.quote == quote)
            .collect::<Vec<&SourceInstrument>>();

        for base_pair in base_contains_instruments {
            for quote_pair in &quote_contains_instruments {
                let to_check = [base_pair.base.clone(), base_pair.quote.clone()];
                if to_check.contains(&quote_pair.base) || to_check.contains(&quote_pair.quote) {
                    return Some((base_pair.clone(), quote_pair.to_owned().clone()));
                }
            }
        }

        return None;
    }
}
#[cfg(test)]
mod tests {
    use service_sdk::rust_extensions::date_time::DateTimeAsMicroseconds;

    use super::*;

    fn create_test_instruments() -> Vec<SourceInstrument> {
        vec![
            SourceInstrument {
                id: "1".to_string(),
                base: "EUR".to_string(),
                quote: "USD".to_string(),
                active_price: CrossMarginBidAsk {
                    asset_pair: "1".to_string(),
                    bid: 1.1,
                    ask: 1.2,
                    base: "EUR".to_string(),
                    quote: "USD".to_string(),
                    date: DateTimeAsMicroseconds::from(123456 as i64),
                },
            },
            SourceInstrument {
                id: "2".to_string(),
                base: "USD".to_string(),
                quote: "JPY".to_string(),
                active_price: CrossMarginBidAsk {
                    asset_pair: "2".to_string(),
                    bid: 110.0,
                    ask: 111.0,
                    base: "USD".to_string(),
                    quote: "JPY".to_string(),
                    date: DateTimeAsMicroseconds::from(123456 as i64),
                },
            },
            SourceInstrument {
                id: "3".to_string(),
                base: "GBP".to_string(),
                quote: "USD".to_string(),
                active_price: CrossMarginBidAsk {
                    asset_pair: "3".to_string(),
                    bid: 1.3,
                    ask: 1.4,
                    base: "GBP".to_string(),
                    quote: "USD".to_string(),
                    date: DateTimeAsMicroseconds::from(123456 as i64),
                },
            },
        ]
    }

    #[test]
    fn test_new_cross_price_engine() {
        let request_crosses = vec![("EUR".to_string(), "JPY".to_string()), ("GBP".to_string(), "USD".to_string())];
        let instruments = create_test_instruments();

        let engine = CrossPriceEngine::new(request_crosses.into_iter(), instruments);

        assert_eq!(engine.cross_matrix.len(), 2);
        assert!(engine.cross_matrix.contains_key("EURJPY"));
        assert!(engine.cross_matrix.contains_key("GBPUSD"));
    }

    #[test]
    fn test_handle_bid_ask() {
        let request_crosses = vec![("EUR".to_string(), "USD".to_string())];
        let instruments = create_test_instruments();
        let mut engine = CrossPriceEngine::new(request_crosses.into_iter(), instruments);

        let new_price = CrossMarginBidAsk {
            asset_pair: "1".to_string(),
            bid: 1.15,
            ask: 1.25,
            base: "EUR".to_string(),
            quote: "USD".to_string(),
            date: DateTimeAsMicroseconds::from(123456 as i64),
        };

        engine.handle_bid_ask(new_price);

        let cross_instrument = engine.get_cross("EUR", "USD").unwrap();
        if let CrossPairType::SameSide(prices) = &cross_instrument.prices {
            assert_eq!(prices.left.bid, 1.1);
            assert_eq!(prices.left.ask, 1.2);
        } else {
            panic!("Expected CrossPairType::SameSide");
        }
    }

    #[test]
    fn test_get_cross() {
        let request_crosses = vec![("EUR".to_string(), "USD".to_string()), ("GBP".to_string(), "USD".to_string())];
        let instruments = create_test_instruments();
        let engine = CrossPriceEngine::new(request_crosses, instruments);

        let eur_usd = engine.get_cross("EUR", "USD");
        assert!(eur_usd.is_some());

        let gbp_usd = engine.get_cross("GBP", "USD");
        assert!(gbp_usd.is_some());

        let usd_jpy = engine.get_cross("USD", "JPY");
        assert!(usd_jpy.is_none());
    }
}
