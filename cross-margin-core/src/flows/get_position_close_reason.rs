use crate::{
    positions::CrossMarginActivePosition, CrossMarginCloseReason, CrossMarginPositionSide,
};

pub fn get_position_close_reason<T: CrossMarginActivePosition>(
    active_position: &T,
) -> Option<CrossMarginCloseReason> {
    if is_sl_triggered(active_position) {
        return Some(CrossMarginCloseReason::Sl);
    }

    if is_tp_triggered(active_position) {
        return Some(CrossMarginCloseReason::Tp);
    }

    return None;
}

fn is_sl_triggered(position: &impl CrossMarginActivePosition) -> bool {
    if let Some(sl) = position.get_sl_profit() {
        return position.get_pl() <= sl;
    }

    if let Some(sl) = position.get_sl_price() {
        return match &position.get_side() {
            CrossMarginPositionSide::Buy => sl >= position.get_active_price(),
            CrossMarginPositionSide::Sell => sl <= position.get_active_price(),
        };
    }

    return false;
}

fn is_tp_triggered(position: &impl CrossMarginActivePosition) -> bool {
    if let Some(tp) = position.get_tp_profit() {
        return position.get_pl() >= tp;
    }

    if let Some(tp) = position.get_tp_price() {
        return match position.get_side() {
            CrossMarginPositionSide::Buy => tp <= position.get_active_price(),
            CrossMarginPositionSide::Sell => tp >= position.get_active_price(),
        };
    }

    return false;
}
