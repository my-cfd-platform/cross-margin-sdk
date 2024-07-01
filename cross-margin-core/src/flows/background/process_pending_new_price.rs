use crate::{
    cache_aggregate::CrossMarginCaches,
    flows::is_pending_ready_to_execute,
    is_enough_balance_to_open_position_sync,
    positions::{
        CrossMarginActivePosition, CrossMarginPendingPosition,
        CrossMarginPendingPositionExecuteReason, CrossMarginPositionsCacheQueryBuilder,
    },
    CrossMarginAccount, CrossMarginBidAsk,
};

pub struct ExecutePendingOrdersResult<P: CrossMarginPendingPosition> {
    pub failed_orders: Vec<(P, CrossMarginPendingPositionExecuteReason)>,
    pub executed_orders: Vec<P>,
}

pub async fn remove_orders_ready_to_execute<
    T: CrossMarginAccount,
    F: CrossMarginActivePosition,
    W: CrossMarginPendingPosition,
>(
    cache: &mut CrossMarginCaches<T, F, W>,
    bid_ask: &CrossMarginBidAsk,
) -> ExecutePendingOrdersResult<W> {
    let pending_cache = &mut cache.pending_positions_cache;
    let active_cache = &mut cache.active_positions_cache;
    let account_cache = &cache.accounts_cache;
    let prices_cache = &cache.prices_cache;

    let removed_orders = pending_cache.query_and_select_remove(
        CrossMarginPositionsCacheQueryBuilder::new()
            .with_base(&bid_ask.base)
            .with_quote(&bid_ask.quote),
        |pending| {
            if is_pending_ready_to_execute(pending, bid_ask) {
                let is_ready_to_open = is_enough_balance_to_open_position_sync(
                    account_cache,
                    active_cache,
                    prices_cache,
                    pending.get_account_id(),
                    pending.get_lots_size(),
                    pending.get_lots_amount(),
                    pending.get_base(),
                );

                let Ok(is_ready_to_open) = is_ready_to_open else {
                    return Some(CrossMarginPendingPositionExecuteReason::Rejected);
                };

                if is_ready_to_open {
                    return Some(CrossMarginPendingPositionExecuteReason::Executed);
                }
            };
            return None;
        },
    );

    let mut result = ExecutePendingOrdersResult {
        failed_orders: vec![],
        executed_orders: vec![],
    };

    for (order, reason) in removed_orders {
        match reason {
            CrossMarginPendingPositionExecuteReason::Executed => result.executed_orders.push(order),
            _ => result.failed_orders.push((order, reason)),
        }
    }

    return result;
}
