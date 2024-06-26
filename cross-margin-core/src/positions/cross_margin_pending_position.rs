use super::CrossMarginPosition;

#[derive(Debug, Clone)]
pub enum CrossMarginPendingPositionType {
    BuyStop = 0,
    BuyLimit = 1,
    SellStop = 2,
    SellLimit = 3,
}

pub enum CrossMarginPendingPositionExecuteReason{
    Cancelled = 0,
    Rejected = 1,
    Executed = 2,
}

pub trait CrossMarginPendingPosition: CrossMarginPosition {
    fn get_desired_price(&self) -> f64;
    fn get_order_type(&self) -> CrossMarginPendingPositionType;
}
