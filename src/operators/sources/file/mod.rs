use anyhow::Result;
use operator::tuples::MappingTuple;

use crate::channels::RcRefChannel;

pub mod csv;

pub trait Source {
    fn create_channel(&mut self) -> Result<RcRefChannel<MappingTuple>>;
}
