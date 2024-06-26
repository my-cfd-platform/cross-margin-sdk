use crate::CrossMarginPositionSide;

use super::CrossMarginCacheIndexGenerator;

pub trait CrossMarginPosition: CrossMarginCacheIndexGenerator + Clone {
    fn get_id(&self) -> &str;
    fn get_trader_id(&self) -> &str;
    fn get_account_id(&self) -> &str;
    fn get_base(&self) -> &str;
    fn get_quote(&self) -> &str;
    fn get_instrument_id(&self) -> &str;
    fn get_collateral(&self) -> &str;
    fn get_side(&self) -> &CrossMarginPositionSide;
    fn get_lots_size(&self) -> f64;
    fn get_lots_amount(&self) -> f64;
    fn get_sl_price(&self) -> Option<f64>;
    fn get_sl_profit(&self) -> Option<f64>;
    fn get_tp_price(&self) -> Option<f64>;
    fn get_tp_profit(&self) -> Option<f64>;

}
