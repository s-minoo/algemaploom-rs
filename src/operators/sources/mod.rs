use operator::{Source as SourceConfig, tuples::MappingTuple};

use super::{BoxedOperatorChainOpt, OperatorChain};
use anyhow::{anyhow, Result};

pub mod file;


pub trait Source: OperatorChain {

    fn create_channel(&mut self) -> Result<crate::channels::RcRefChannel<MappingTuple>> ;
    
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
    }
}
