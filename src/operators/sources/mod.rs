use std::sync::Arc;

use anyhow::{Result};
use operator::tuples::MappingTuple;
use operator::Source as SourceConfig;

use super::{BoxedOperatorChainOpt};
use crate::channels::Channel;

pub mod file;

pub trait Source {
    fn create_channel(&mut self) -> Result<Channel<MappingTuple>>;

    fn output(&mut self) -> Vec<Arc<Channel<MappingTuple>>>;

    async fn execute(&mut self);
}

pub fn to_physical_source(
    logical_source: SourceConfig,
) -> BoxedOperatorChainOpt {
    match logical_source.source_type {
        operator::IOType::File => todo!(),
        operator::IOType::Kafka => todo!(),
        operator::IOType::Websocket => todo!(),
        operator::IOType::MySQL => todo!(),
        operator::IOType::PostgreSQL => todo!(),
        operator::IOType::SPARQLEndpoint => todo!(),
        operator::IOType::StdOut => todo!(),
    }
}
