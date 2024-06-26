use std::collections::HashMap;

use service_sdk::rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{
    flows::{calculate_account_data, AccountCalculationResult},
    positions::CrossMarginActivePosition,
};

pub trait CrossMarginAccount: Clone {
    fn get_trader_id(&self) -> &str;
    fn get_id(&self) -> &str;
    fn get_stop_out(&self) -> f64;
    fn get_balance(&self) -> f64;
    fn get_currency(&self) -> &str;
    fn get_leverage(&self) -> f64;
    fn get_instruments_leverages(&self) -> &HashMap<String, f64>;
    fn update_balance(&mut self, delta: f64);
    fn update_trading_group(&mut self, new_group: String);
    fn update_leverage(&mut self, leverage: f64);
    fn set_trading_disabled(&mut self, disabled: bool);
    fn track_update(&mut self, process_id: &str, date: DateTimeAsMicroseconds);
    fn calculate_account_margin_props(
        &self,
        positions: &Vec<&impl CrossMarginActivePosition>,
    ) -> AccountCalculationResult {
        return calculate_account_data(self, positions);
    }
}
