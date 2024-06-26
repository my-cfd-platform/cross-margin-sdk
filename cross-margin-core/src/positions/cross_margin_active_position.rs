use crate::{CrossMarginBidAsk, CrossMarginCloseReason};

use super::{CrossMarginCacheIndexGenerator, CrossMarginClosedPosition, CrossMarginPosition};

pub trait CrossMarginActivePosition:
    Clone + CrossMarginCacheIndexGenerator + CrossMarginPosition
{
    fn get_pl(&self) -> f64;
    fn update_pl(&mut self, pl: f64);
    fn get_open_price(&self) -> f64;
    fn get_active_price(&self) -> f64;
    fn get_profit_price(&self) -> f64;
    fn get_margin_price(&self) -> f64;
    fn update_profit_price(&mut self, bid_ask: CrossMarginBidAsk, price: f64);
    fn update_asset_price(&mut self, bid_ask: CrossMarginBidAsk, price: f64);
}
