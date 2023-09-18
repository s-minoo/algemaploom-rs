use serde::{Deserialize, Serialize};

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
