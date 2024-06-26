use crate::{positions::CrossMarginActivePosition, CrossMarginAccount};

pub fn is_account_stop_out_hit(
    account: &impl CrossMarginAccount,
    account_positions: &Vec<&impl CrossMarginActivePosition>,
) -> bool {
    let account_props = account
        .calculate_account_margin_props(account_positions);

    return account_props.margin_level <= account.get_stop_out();
}
