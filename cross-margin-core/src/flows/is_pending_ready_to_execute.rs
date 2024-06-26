use crate::{positions::CrossMarginPendingPosition, CrossMarginBidAsk};

pub fn is_pending_ready_to_execute<T: CrossMarginPendingPosition>(
    position: &T,
    new_bid_ask: &CrossMarginBidAsk,
) -> bool {
    match position.get_order_type() {
        crate::positions::CrossMarginPendingPositionType::BuyStop => {
            new_bid_ask.get_open_price(&crate::CrossMarginPositionSide::Buy)
                >= position.get_desired_price()
        }
        crate::positions::CrossMarginPendingPositionType::BuyLimit => {
            new_bid_ask.get_open_price(&crate::CrossMarginPositionSide::Buy)
                <= position.get_desired_price()
        }
        crate::positions::CrossMarginPendingPositionType::SellStop => {
            new_bid_ask.get_open_price(&crate::CrossMarginPositionSide::Sell)
                >= position.get_desired_price()
        }
        crate::positions::CrossMarginPendingPositionType::SellLimit => {
            new_bid_ask.get_open_price(&crate::CrossMarginPositionSide::Sell)
                <= position.get_desired_price()
        }
    }
}
