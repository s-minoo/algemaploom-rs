use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum ReferenceFormulation {
    CSVRows,
    JSONPath, 
    XMLPath,
    XMLQuery,
    SQLQuery, 
    SPARQL
}
impl Default for ReferenceFormulation {
    fn default() -> Self {
        ReferenceFormulation::CSVRows
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum DataFormat {
    JSONLD,
    JSON,
    XML,
    CSV,
    TTL,
    NQuads,
    NTriples,
    SQL,
}

impl Default for DataFormat {
    fn default() -> Self {
        DataFormat::CSV
    }
}
