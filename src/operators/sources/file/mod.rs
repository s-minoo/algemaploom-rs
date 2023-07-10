use anyhow::Result;
use operator::tuples::MappingTuple;

use crate::channels::RcChannel;

pub mod csv;

pub trait Source {
    fn create_channel(&mut self) -> Result<RcChannel<MappingTuple>>;
}
