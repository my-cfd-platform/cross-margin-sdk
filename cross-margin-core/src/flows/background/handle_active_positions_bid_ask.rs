use crate::{
    cache_aggregate::CrossMarginCaches,
    flows::{get_position_close_reason, update_position_rates},
    positions::{
        CrossMarginActivePosition, CrossMarginPendingPosition,
        CrossMarginPositionsOneOfBulkQueryBuilder,
    },
    CrossMarginAccount, CrossMarginBidAsk, CrossMarginCloseReason,
};


#[derive(Debug, Clone)]
pub struct UpdatePositionsDto {
    pub trader_id: String,
    pub account_id: String,
    pub position_id: String,
    pub close_position_reason: Option<CrossMarginCloseReason>,
}

pub fn update_active_positions_rates<
    T: CrossMarginAccount,
    F: CrossMarginActivePosition,
    W: CrossMarginPendingPosition,
>(
    caches: &mut CrossMarginCaches<T, F, W>,
    new_bid_ask: &CrossMarginBidAsk,
) -> Vec<UpdatePositionsDto> {
    let search = vec![new_bid_ask.base.clone(), new_bid_ask.quote.clone()];

    let update_function = |position: &mut F| {
        if let Err(err) = update_position_rates(position, &caches.prices_cache) {
            panic!(
                "Error to update position rates: {}. Err: {:?}",
                position.get_id(),
                err
            )
        }

        return Some(UpdatePositionsDto {
            trader_id: position.get_trader_id().to_string(),
            account_id: position.get_account_id().to_string(),
            position_id: position.get_id().to_string(),
            close_position_reason: get_position_close_reason(position),
        });
    };

    return caches.active_positions_cache.bulk_update_positions(
        CrossMarginPositionsOneOfBulkQueryBuilder::new()
            .with_base(search.clone())
            .with_quote(search.clone())
            .with_collateral(search.clone()),
        update_function,
    );
}
