use crate::{
    positions::{CrossMarginActivePosition, CrossMarginPositionsCacheQueryBuilder, PositionsCache},
    AccountsCache, CrossMarginAccount, CrossMarginBidAskCache, CrossMarginError,
    CrossMarginPositionSide,
};

pub fn is_enough_balance_to_open_position<A: CrossMarginAccount, AP: CrossMarginActivePosition>(
    accounts_cache: &AccountsCache<A>,
    active_positions_cache: &PositionsCache<AP>,
    prices_cache: &CrossMarginBidAskCache,
    account_id: &str,
    lots_size: f64,
    lots_amount: f64,
    base: &str,
) -> Result<bool, CrossMarginError> {
    let account = accounts_cache
        .get_account(account_id)
        .ok_or(CrossMarginError::AccountNotFound)?;

    let account_positions = active_positions_cache
        .query_positions(CrossMarginPositionsCacheQueryBuilder::new().with_account(account_id));

    let account_props = account.calculate_account_margin_props(&account_positions);
    let margin_bid_ask = prices_cache.get_price(base, account.get_currency()).ok_or(
        CrossMarginError::AssetNotFound(format!(
            "{}-{} for account {} NOT FOUND",
            base,
            account.get_currency(),
            account_id
        )),
    )?;

    let new_position_margin = lots_size * lots_amount / account.get_leverage()
        * margin_bid_ask.get_open_price(&CrossMarginPositionSide::Buy);

    let required_margin = lots_size * lots_amount / account.get_leverage() * new_position_margin;
    return Ok(account_props.free_margin >= required_margin);
}
