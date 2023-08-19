use std::collections::HashMap;

use anyhow::{anyhow, Result};
use csv::Reader;
use operator::tuples::{MappingTuple, SolutionMapping};
use operator::Source as SourceConfig;

use super::{FileSource, Source};
use crate::channels::Channel;
use crate::operators::{BoxedOperatorChainOpt, OperatorChain};

pub struct CSVFileSource {
    pub config: SourceConfig,
    pub next:   BoxedOperatorChainOpt,
}

impl OperatorChain for CSVFileSource {
    fn into_boxed_opt(self) -> crate::operators::BoxedOperatorChainOpt {
        Some(Box::new(self))
    }

    fn next(&mut self) -> &mut crate::operators::BoxedOperatorChainOpt {
        &mut self.next
    }

    fn process_solution_mapping(&mut self, mapping: &mut SolutionMapping) {
        todo!()
    }
}

impl Source for CSVFileSource {
    fn create_channel(&mut self) -> Result<Channel<MappingTuple>> {
        let file_path = self.config.config.get("path").ok_or(anyhow!(
            "Path doesn't exist in the source configuration {:?}",
            self.config
        ))?;

        let mut reader = Reader::from_path(file_path)?;
        let attributes: Vec<_> =
            reader.headers()?.iter().map(|i| i.to_string()).collect();

        let mapping_tuple_iter = reader
            .into_records()
            .filter_map(|record_res| record_res.ok())
            .map(move |record| {
                let zipped: SolutionMapping = attributes
                    .iter()
                    .map(|attr| attr.to_owned())
                    .zip(record.iter().map(|r| r.into()))
                    .collect();
                zipped
            })
            .map(|solution_mapping| {
                let map_tuple: MappingTuple = HashMap::from([(
                    "default".to_string(),
                    vec![solution_mapping],
                )]);

                map_tuple
            });
        todo!()
    }

    fn output(&mut self) -> Vec<std::sync::Arc<Channel<MappingTuple>>> {
        todo!()
    }

    async fn execute(&mut self) {
        todo!()
    }
}

impl FileSource for CSVFileSource {
    fn file(&self) -> std::path::PathBuf {
        todo!()
    }

    fn close(&mut self) {
        todo!()
    }
}
