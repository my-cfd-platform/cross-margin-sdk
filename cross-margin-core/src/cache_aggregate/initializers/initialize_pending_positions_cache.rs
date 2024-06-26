use crate::{
    positions::{CrossMarginPendingPosition, PositionsCache},
    CrossMarginError,
};

pub async fn initialize_pending_cache<T: CrossMarginPendingPosition>(
    pending_positions: Vec<T>,
) -> Result<PositionsCache<T>, Vec<CrossMarginError>> {

    return Ok(PositionsCache::new("PendingPositions".to_string(), pending_positions));

}
