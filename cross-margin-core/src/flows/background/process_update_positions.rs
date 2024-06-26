use std::collections::HashSet;

use crate::{
    cache_aggregate::CrossMarginCaches,
    flows::is_account_stop_out_hit,
    positions::{
        CrossMarginActivePosition, CrossMarginPendingPosition,
        CrossMarginPositionsCacheQueryBuilder,
    },
    CrossMarginAccount, CrossMarginCloseReason,
};

use super::UpdatePositionsDto;

pub async fn process_positions_update<
    T: CrossMarginAccount,
    F: CrossMarginActivePosition,
    W: CrossMarginPendingPosition,
>(
    cache: &mut CrossMarginCaches<T, F, W>,
    updated_positions: Vec<UpdatePositionsDto>,
    process_id: &str,
) -> Vec<(F, CrossMarginCloseReason)> {
    let mut positions_to_close = vec![];
    let mut updated_accounts = HashSet::new();

    for update in updated_positions {
        updated_accounts.insert(update.account_id.clone());
        if let Some(close_reason) = update.close_position_reason {
            positions_to_close.push((update.position_id.clone(), close_reason));
        }
    }

    let mut removed_positions = cache
        .remove_active_positions(positions_to_close.as_slice(), process_id)
        .await;

    let so_accounts_with_max_loss = updated_accounts.iter().filter_map(|x| {
        let account = cache.accounts_cache.get_account(x)?;

        let account_positions = cache
            .active_positions_cache
            .query_positions(CrossMarginPositionsCacheQueryBuilder::new().with_account(&x));

        if is_account_stop_out_hit(account, &account_positions) {
            if let Some(max_loss_position) = account_positions
                .iter()
                .min_by(|x, y| x.get_pl().partial_cmp(&y.get_pl()).unwrap())
            {
                return Some((account, max_loss_position.to_owned()));
            }
        };

        return None;
    });

    let so_positions_to_close: Vec<(String, CrossMarginCloseReason)> = so_accounts_with_max_loss
        .map(|(_, ap)| (ap.get_id().to_string(), CrossMarginCloseReason::StopOut))
        .collect();

    let so_removed_positions = cache
        .remove_active_positions(&so_positions_to_close, process_id)
        .await;

    removed_positions.extend(so_removed_positions);

    return removed_positions;
}
