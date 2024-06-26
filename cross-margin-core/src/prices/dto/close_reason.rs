#[derive(Debug, Clone)]
pub enum CrossMarginCloseReason{
    Sl,
    Tp,
    ClientCommand,
    StopOut,
    AdminClose
}