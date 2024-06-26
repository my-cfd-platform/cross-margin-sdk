use crate::{
    flows::update_position_rates,
    positions::{CrossMarginActivePosition, PositionsCache},
    CrossMarginBidAskCache, CrossMarginError,
};

pub async fn initialize_active_positions_cache<T: CrossMarginActivePosition>(
    raw_positions: Vec<T>,
    cache: &CrossMarginBidAskCache,
) -> Result<PositionsCache<T>, Vec<CrossMarginError>> {
    let mut active_positions = vec![];
    let mut errors = vec![];

    for mut position in raw_positions {
        match update_position_rates(&mut position, cache) {
            Ok(_) => active_positions.push(position),
            Err(err) => errors.push(err),
        }
    }

    if errors.is_empty() {
        Ok(PositionsCache::new("ActivePositions".to_string(), active_positions))
    } else {
        Err(errors)
    }
}
