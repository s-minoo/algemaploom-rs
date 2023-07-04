pub mod formats;
mod test_util;
pub mod tuples;
pub mod value;

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use formats::DataFormat;
use serde::{Deserialize, Serialize};

pub type RcOperator = Rc<Operator>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Operator {
    SourceOp(Source),
    TransformOp {
        config:   Transform,
        operator: RcOperator,
    },
    JoinOp {
        config:    Join,
        operators: Vec<RcOperator>,
    },
    ProjectOp {
        config:   Projection,
        operator: RcOperator,
    },
    ExtendOp {
        config:   Extend,
        operator: RcOperator,
    },
    RenameOp {
        config:   Rename,
        operator: RcOperator,
    },
    SerializerOp {
        config:   Serializer,
        operator: RcOperator,
    },
    TargetOp {
        config:   Target,
        operator: RcOperator,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Source {
    pub configuration: HashMap<String, String>,
    pub source_type:   IOType,
    pub data_format:   DataFormat,
}

// Transformation operators
/// Alias type to define Foreign Function Interface (FFI) configurations.
pub type FFIConfig = HashMap<String, String>;

/// Enums for transformation operators where the data item can be
/// processed/transformed through the use of FFI's or built-in functions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Transform {
    ArbitraryTransform(FFIConfig),
    Lower(String),
    Upper(String),
}

////

// Join operators

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum JoinType {
    LeftJoin,
    EquiJoin,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Join {
    pub left_right_pairs: HashMap<String, String>,
    pub join_type:        JoinType,
}
impl Join {
    pub fn is_binary_join(&self) -> bool {
        // TODO:  <30-05-23, Sitt Min Oo> //

        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Projection {
    pub projection_attributes: HashSet<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rename {
    pub rename_pairs: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Extend {
    pub extend_pairs: HashMap<String, Function>,
}

pub type RcExtendFunction = Rc<Function>;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Function {
    Reference(String),
    Constant(String),
    Template(String),
    UriEncode(RcExtendFunction),
    Iri(RcExtendFunction),
    Literal(RcExtendFunction),
    BlankNode(RcExtendFunction),
    Upper(RcExtendFunction),
    Lower(RcExtendFunction),
}

// Post-mapping operators

// TODO: Unit struct for now since I have
// no idea which fields are required for the
// serializer component <26-04-23, Min Oo> //
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Serializer {
    pub template: String,
    pub options:  Option<HashMap<String, String>>,
    pub format:   DataFormat,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Target {
    pub configuration: HashMap<String, String>,
    pub target_type:   IOType,
    pub data_format:   DataFormat,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IOType {
    File,
    Kafka,
    Websocket,
    MySQL,
    PostgreSQL,
    SPARQLEndpoint,
}
