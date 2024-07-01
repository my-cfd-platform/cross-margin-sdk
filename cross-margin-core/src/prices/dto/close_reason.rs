use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CrossMarginCloseReason{
    Sl,
    Tp,
    ClientCommand,
    StopOut,
    AdminClose
}