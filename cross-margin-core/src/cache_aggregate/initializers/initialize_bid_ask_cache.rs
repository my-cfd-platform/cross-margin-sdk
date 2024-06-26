use std::collections::{HashMap, HashSet};

use crate::{
    cache_aggregate::CrossMarginCacheInstrument, CrossMarginBidAsk, CrossMarginBidAskCache,
    SourceInstrument,
};

pub async fn initialize_bid_ask_cache(
    instruments: Vec<CrossMarginCacheInstrument>,
    collaterals: Vec<String>,
    prices: Vec<CrossMarginBidAsk>,
) -> CrossMarginBidAskCache {
    let mut crosses = HashSet::new();
    let prices_snapshot = prices
        .into_iter()
        .map(|x| (x.asset_pair.clone(), x))
        .collect::<HashMap<String, CrossMarginBidAsk>>();

    for instrument in instruments.clone() {
        for collateral in collaterals.clone() {
            if instrument.base != collateral {
                crosses.insert((instrument.base.clone(), collateral.clone()));
            }
            if instrument.quote != collateral {
                crosses.insert((instrument.quote.clone(), collateral.clone()));
            }
        }
    }

    let mapped_instruments = instruments
        .iter()
        .map(|x| SourceInstrument {
            id: x.id.clone(),
            base: x.base.clone(),
            quote: x.quote.clone(),
            active_price: prices_snapshot
                .get(&x.id)
                .map(|x| x.to_owned())
                .expect(format!("No price for instrument {}", x.id).as_str())
                .clone(),
        })
        .collect::<Vec<_>>();

    return CrossMarginBidAskCache::new(
        crosses,
        mapped_instruments,
        prices_snapshot.into_iter().map(|x| x.1).collect(),
    );
}
