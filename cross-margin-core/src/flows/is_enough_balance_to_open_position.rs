use service_sdk::my_telemetry::MyTelemetryContext;

use crate::{
    positions::{CrossMarginActivePosition, CrossMarginPositionsCacheQueryBuilder, PositionsCache},
    AccountsCache, CrossMarginAccount, CrossMarginBidAskCache, CrossMarginError,
    CrossMarginPositionSide,
};

pub async fn is_enough_balance_to_open_position<
    A: CrossMarginAccount,
    AP: CrossMarginActivePosition,
>(
    accounts_cache: &AccountsCache<A>,
    active_positions_cache: &PositionsCache<AP>,
    prices_cache: &CrossMarginBidAskCache,
    account_id: &str,
    lots_size: f64,
    lots_amount: f64,
    base: &str,
    instrument_id: &str
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

    let instrument_leverage = account.get_instruments_leverages().get(instrument_id).map(|x| x.clone()).unwrap_or(account.get_leverage());

    let target_leverage = instrument_leverage.min(account.get_leverage());

    let new_position_margin = lots_size * lots_amount / target_leverage
        * margin_bid_ask.get_open_price(&CrossMarginPositionSide::Buy);

    let required_margin = lots_size * lots_amount / target_leverage * new_position_margin;

    trade_log::trade_log!(
        account.get_trader_id(),
        account.get_id(),
        "N/A",
        "N/A",
        "Validation is enough balance to open position",
        MyTelemetryContext::new().clone(),
        "account" = &account,
        "account_positions" = &account_positions,
        "account_props" = &account_props,
        "margin_bid_ask" = margin_bid_ask.as_ref(),
        "new_position_margin" = &new_position_margin,
        "required_margin" = &required_margin,
        "target_leverage" = &target_leverage
    );

    return Ok(account_props.free_margin >= required_margin);
}



pub fn is_enough_balance_to_open_position_sync<
    A: CrossMarginAccount,
    AP: CrossMarginActivePosition,
>(
    accounts_cache: &AccountsCache<A>,
    active_positions_cache: &PositionsCache<AP>,
    prices_cache: &CrossMarginBidAskCache,
    account_id: &str,
    lots_size: f64,
    lots_amount: f64,
    base: &str,
    instrument_id: &str
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

    let instrument_leverage = account.get_instruments_leverages().get(instrument_id).map(|x| x.clone()).unwrap_or(account.get_leverage());

    let target_leverage = instrument_leverage.min(account.get_leverage());

    let new_position_margin = lots_size * lots_amount / target_leverage
        * margin_bid_ask.get_open_price(&CrossMarginPositionSide::Buy);

    let required_margin = lots_size * lots_amount / target_leverage * new_position_margin;

    return Ok(account_props.free_margin >= required_margin);
}