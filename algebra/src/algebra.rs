use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Operator {
    SourceOp(Source, Rc<Operator>),
    MappingOp(Mapping, Rc<Operator>),
    TargetOp(Target),
}


#[derive(Debug, Clone)]
pub struct Source {
    pub configuration: HashMap<String, String>,
    pub source_type:   IOType,
    pub data_format:   DataFormat,
}

#[derive(Debug, Clone)]
pub struct Mapping {
    pub mapping_document: PathBuf,
}

#[derive(Debug, Clone)]
pub struct Target {
    pub configuration: HashMap<String, String>,
    pub target_type:   IOType,
}


pub struct DataItem {
    pub fields_value: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub enum DataFormat {
    JSON,
    XML,
    CSV,
    TTL,
    NQ,
    SQL,
}

#[derive(Debug, Clone)]
pub enum IOType {
    File,
    Kafka,
    Websocket,
    MySQL,
    PostgreSQL,
    SPARQLEndpoint,
}


