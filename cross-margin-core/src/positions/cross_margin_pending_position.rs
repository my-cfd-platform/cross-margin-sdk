use serde::{de::DeserializeOwned, Deserialize, Serialize};

use super::CrossMarginPosition;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CrossMarginPendingPositionType {
    BuyStop = 0,
    BuyLimit = 1,
    SellStop = 2,
    SellLimit = 3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CrossMarginPendingPositionExecuteReason{
    Cancelled = 0,
    Rejected = 1,
    Executed = 2,
}

pub trait CrossMarginPendingPosition: CrossMarginPosition + Serialize + DeserializeOwned + Clone{
    fn get_desired_price(&self) -> f64;
    fn get_order_type(&self) -> CrossMarginPendingPositionType;
}
