use std::collections::HashMap;
use std::path::PathBuf;

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
    pub fields_value: HashMap<String, String>,
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


