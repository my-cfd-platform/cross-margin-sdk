mod prices;
mod accounts;
mod positions;
mod cache_aggregate;
mod flows;

pub use accounts::*;
pub use prices::*;
pub use positions::*;
pub use cache_aggregate::*;
pub use flows::*;

#[derive(Debug)]
pub enum CrossMarginError{
    AccountNotFound,
    PositionNotFound,
    NotEnoughBalance,
    AssetNotFound(String),
    MultiError(Vec<String>),
}