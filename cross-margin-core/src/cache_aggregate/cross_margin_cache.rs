use crate::{
    accounts::CrossMarginAccount,
    flows::{
        is_enough_balance_to_open_position, process_positions_update,
        remove_orders_ready_to_execute, update_active_positions_rates,
    },
    positions::{
        CrossMarginActivePosition, CrossMarginPendingPosition,
        CrossMarginPendingPositionExecuteReason, PositionsCache,
    },
    AccountsCache, CrossMarginBidAsk, CrossMarginBidAskCache, CrossMarginCloseReason,
    CrossMarginError,
};

use super::{
    initialize_account_cache, initialize_active_positions_cache, initialize_bid_ask_cache,
    initialize_pending_cache,
};

pub struct CrossMarginCacheHandleBidAskResult<
    AP: CrossMarginActivePosition,
    PP: CrossMarginPendingPosition,
> {
    pub closed_positions: Vec<(AP, CrossMarginCloseReason)>,
    pub failed_orders: Vec<(PP, CrossMarginPendingPositionExecuteReason)>,
    pub executed_orders: Vec<PP>,
}

#[derive(Clone, Debug)]
pub struct CrossMarginCacheInstrument {
    pub id: String,
    pub base: String,
    pub quote: String,
}

pub struct CrossMarginCaches<A, AP, PP>
where
    A: CrossMarginAccount,
    AP: CrossMarginActivePosition,
    PP: CrossMarginPendingPosition,
{
    pub prices_cache: CrossMarginBidAskCache,
    pub accounts_cache: AccountsCache<A>,
    pub active_positions_cache: PositionsCache<AP>,
    pub pending_positions_cache: PositionsCache<PP>,
}

impl<A, AP, PP> CrossMarginCaches<A, AP, PP>
where
    A: CrossMarginAccount,
    AP: CrossMarginActivePosition,
    PP: CrossMarginPendingPosition,
{
    pub async fn new(
        accounts: Vec<A>,
        active_positions: Vec<AP>,
        pending_positions: Vec<PP>,
        instruments: Vec<CrossMarginCacheInstrument>,
        collaterals: Vec<String>,
        prices: Vec<CrossMarginBidAsk>,
    ) -> Result<Self, CrossMarginError> {
        let bid_ask_cache = initialize_bid_ask_cache(instruments, collaterals, prices).await;
        let accounts_cache = initialize_account_cache(accounts).await;

        let active_cache =
            match initialize_active_positions_cache(active_positions, &bid_ask_cache).await {
                Ok(src) => src,
                Err(errs) => panic!(
                    "Multiple errors: {:?} during active cache initializations",
                    errs
                ),
            };

        let pending_positions_cache = match initialize_pending_cache(pending_positions).await {
            Ok(src) => src,
            Err(errs) => panic!(
                "Multiple errors: {:?} during pending cache initializations",
                errs
            ),
        };

        return Ok(CrossMarginCaches {
            prices_cache: bid_ask_cache,
            accounts_cache,
            active_positions_cache: active_cache,
            pending_positions_cache,
        });
    }

    pub async fn is_enough_balance_to_open_position(
        &self,
        account_id: &str,
        lots_size: f64,
        lots_amount: f64,
        base: &str,
    ) -> Result<bool, CrossMarginError> {
        return is_enough_balance_to_open_position(
            &self.accounts_cache,
            &self.active_positions_cache,
            &self.prices_cache,
            account_id,
            lots_size,
            lots_amount,
            base,
        )
        .await;
    }

    pub async fn handle_bid_ask(
        &mut self,
        bid_ask: CrossMarginBidAsk,
        process_id: &str,
    ) -> CrossMarginCacheHandleBidAskResult<AP, PP> {
        self.prices_cache.handle_new(bid_ask.clone());
        let updated_positions = update_active_positions_rates(self, &bid_ask);
        let closed_positions = process_positions_update(self, updated_positions, process_id).await;
        let executed_limits_orders = remove_orders_ready_to_execute(self, &bid_ask).await;

        return CrossMarginCacheHandleBidAskResult {
            closed_positions,
            failed_orders: executed_limits_orders.failed_orders,
            executed_orders: executed_limits_orders.executed_orders,
        };
    }

    pub async fn add_active_position(
        &mut self,
        position: AP,
        _: &str,
    ) -> Result<(), CrossMarginError> {
        self.active_positions_cache.add_position(position.clone());
        return Ok(());
    }

    pub async fn remove_active_position(
        &mut self,
        id: &str,
        process_id: &str,
    ) -> Result<(AP, A), CrossMarginError> {
        let removed_position = self
            .active_positions_cache
            .remove_position(id)
            .ok_or(CrossMarginError::PositionNotFound)?;

        let account_after_update = self
            .accounts_cache
            .update_balance(
                removed_position.get_account_id(),
                removed_position.get_pl(),
                process_id,
                true,
            )
            .await?;

        return Ok((removed_position, account_after_update));
    }

    pub async fn remove_active_positions(
        &mut self,
        ids: &[(String, CrossMarginCloseReason)],
        process_id: &str,
    ) -> Vec<(AP, CrossMarginCloseReason)> {
        let removed_positions: Vec<(AP, CrossMarginCloseReason)> = ids
            .into_iter()
            .filter_map(|(id, close_reason)| {
                if let Some(removed_position) = self.active_positions_cache.remove_position(id) {
                    return Some((removed_position, close_reason.clone()));
                }

                return None;
            })
            .collect();

        for (position, _) in removed_positions.iter() {
            self.accounts_cache
                .update_balance(
                    position.get_account_id(),
                    position.get_pl(),
                    process_id,
                    true,
                )
                .await
                .unwrap();
        }

        return removed_positions;
    }
}
