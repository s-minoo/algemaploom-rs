mod test_util;
pub mod value;

use std::collections::HashMap;
use std::path::PathBuf;

use value::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    SourceOp(Source, Box<Operator>),
    MappingOp(Mapping, Box<Operator>),
    TargetOp(Target),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Source {
    pub configuration: HashMap<String, String>,
    pub source_type:   IOType,
    pub data_format:   DataFormat,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Mapping {
    pub mapping_document: PathBuf,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Target {
    pub configuration: HashMap<String, String>,
    pub target_type:   IOType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DataItem {
    pub fields_value: HashMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MappedDataItem {
    pub id_key:         String,
    pub id_val:         Value,
    pub attributes_map: HashMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataFormat {
    JSON,
    XML,
    CSV,
    TTL,
    NQ,
    SQL,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IOType {
    File,
    Kafka,
    Websocket,
    MySQL,
    PostgreSQL,
    SPARQLEndpoint,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_simple_rml() {
        let file = test_resource!("sample_mapping.ttl");

        let chains = Operator::SourceOp(
            Source {
                configuration: HashMap::from([(
                    "path".into(),
                    "Airport.csv".into(),
                )]),
                source_type:   IOType::File,
                data_format:   DataFormat::CSV,
            },
            Box::new(Operator::MappingOp(
                Mapping {
                    mapping_document: file.into(),
                },
                Box::new(Operator::TargetOp(Target {
                    configuration: HashMap::from([(
                        "path".into(),
                        "output.nt".into(),
                    )]),
                    target_type:   IOType::File,
                })),
            )),
        );

        println!("{:#?}", chains);
    }
}
