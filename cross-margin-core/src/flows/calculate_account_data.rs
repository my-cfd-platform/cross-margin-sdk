use crate::{positions::CrossMarginActivePosition, CrossMarginAccount};

use super::calculate_margin;

#[derive(Debug, Clone)]
pub struct AccountCalculationResult {
    pub margin: f64,
    pub equity: f64,
    pub free_margin: f64,
    pub margin_level: f64,
}

pub fn calculate_account_data(
    account: &impl CrossMarginAccount,
    positions: &Vec<&impl CrossMarginActivePosition>,
) -> AccountCalculationResult {
    let margin = calculate_margin(account, positions);
    let equity = account.get_balance() + positions.iter().map(|x| x.get_pl()).sum::<f64>();
    AccountCalculationResult {
        margin,
        equity,
        free_margin: equity - margin,
        margin_level: match margin < 0.0001 {
            true => 0.0,
            false => equity / margin * 100.0,
        },
    }
}
