use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash)]
pub enum DataFormat {
    JSON,
    XML,
    CSV,
    TTL,
    NQ,
    NT,
    SQL,
}
