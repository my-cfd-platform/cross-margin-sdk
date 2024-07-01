use crate::{
    positions::CrossMarginActivePosition, CrossMarginBidAskCache, CrossMarginError,
    CrossMarginPositionSide,
};

pub fn update_position_rates(
    active_position: &mut impl CrossMarginActivePosition,
    cache: &CrossMarginBidAskCache,
) -> Result<(), CrossMarginError> {
    let profit_bid_ask = cache
        .get_price(
            active_position.get_quote(),
            active_position.get_collateral(),
        )
        .ok_or(CrossMarginError::AssetNotFound(format!(
            "{}-{} for position {} NOT FOUND",
            active_position.get_quote(),
            active_position.get_collateral(),
            active_position.get_id()
        )))?;

    let asset_price = cache.get_by_id(active_position.get_instrument_id()).ok_or(
        CrossMarginError::AssetNotFound(format!(
            "{} for position {} NOT FOUND",
            active_position.get_instrument_id(),
            active_position.get_id()
        )),
    )?;

    let profit_rate = match active_position.get_pl() > 0.0 {
        true => profit_bid_ask.bid,
        false => profit_bid_ask.ask,
    };

    active_position.update_profit_price(profit_bid_ask.as_ref().clone(), profit_rate);
    active_position.update_asset_price(
        asset_price.as_ref().clone(),
        asset_price.get_close_price(active_position.get_side()),
    );

    let open_side = active_position.get_open_price()
        * active_position.get_lots_size()
        * active_position.get_lots_amount();
    let close_side = active_position.get_active_price()
        * active_position.get_lots_size()
        * active_position.get_lots_amount();

    let pl = match active_position.get_side() {
        &CrossMarginPositionSide::Buy => close_side - open_side,
        &CrossMarginPositionSide::Sell => open_side - close_side,
    } * active_position.get_profit_price();

    active_position.update_pl(pl);

    return Ok(());
}
