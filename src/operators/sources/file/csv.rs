use std::collections::HashMap;

use anyhow::{anyhow, Result};
use csv::Reader;
use operator::tuples::{MappingTuple, SolutionMapping};
use operator::Source as SourceConfig;

use super::Source;
use crate::channels::{Channel, RcChannel};

#[derive(Debug, Clone)]
pub struct CSVFileSource {
    config: SourceConfig,
}

impl Source for CSVFileSource {
    fn create_channel(&mut self) -> Result<RcChannel<MappingTuple>> {
        let file_path =
            self.config.config.get("path").ok_or(anyhow!(
                "Path doesn't exist in the source configuration {:?}",
                self
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
                    .zip(record.iter().map(|r| vec![r.into()]))
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

        Ok(Channel::new_rc(Box::new(mapping_tuple_iter)))
    }
}
