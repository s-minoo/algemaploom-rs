use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum AlgebraType<T> {
    Source(Source),
    Mapping(Mapping<T>),
    Target(Target),
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

#[derive(Debug, Clone)]
pub struct Source {
    pub configuration: HashMap<String, String>,
    pub source_type:   IOType,
    pub data_format:   DataFormat,
}

#[derive(Debug, Clone)]
pub struct Mapping<T> {
    pub mapping_document: T,
}



#[derive(Debug, Clone)]
pub struct Target {
    pub configuration: HashMap<String, String>,
    pub target_type: IOType ,
}


