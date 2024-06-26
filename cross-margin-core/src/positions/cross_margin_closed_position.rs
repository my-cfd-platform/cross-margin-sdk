use super::{CrossMarginCacheIndexGenerator, CrossMarginPosition};

pub trait CrossMarginClosedPosition:
    Clone + CrossMarginCacheIndexGenerator + CrossMarginPosition
{
    
}