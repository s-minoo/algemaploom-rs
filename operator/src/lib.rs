pub mod formats;
pub mod plan;
mod test_util;
pub mod tuples;
pub mod value;

use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use formats::DataFormat;
use serde::{Deserialize, Serialize};

pub type RcOperator = Rc<Operator>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash)]
#[serde(tag = "type")]
pub enum Operator {
    SourceOp(Source),
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

fn hash_hashmap<H, K, V>(hash_map: &HashMap<K, V>, state: &mut H)
where
    H: Hasher,
    K: Hash + Ord,
    V: Hash,
{
    let mut pairs: Vec<_> = hash_map.iter().collect();
    pairs.sort_by(|pair1, pair2| pair1.0.cmp(pair2.0));
    for (key, value) in pairs {
        key.hash(state);
        value.hash(state);
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Source {
    pub config:      HashMap<String, String>,
    pub source_type: IOType,
    pub data_format: DataFormat,
}

impl Hash for Source {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        hash_hashmap(&self.config, state);
        self.source_type.hash(state);
        self.data_format.hash(state);
    }
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

impl Hash for Transform {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);

        match self {
            Transform::ArbitraryTransform(data) => hash_hashmap(&data, state),
            Transform::Lower(data) => data.hash(state),
            Transform::Upper(data) => data.hash(state),
        }
    }
}

////

// Join operators

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash)]
pub enum JoinType {
    LeftJoin,
    EquiJoin,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Join {
    pub left_right_pairs: HashMap<String, String>,
    pub join_type:        JoinType,
}

impl Hash for Join {
    fn hash<H: Hasher>(&self, state: &mut H) {
        hash_hashmap(&self.left_right_pairs, state);
        self.join_type.hash(state);
    }
}

impl Join {
    pub fn is_binary_join(&self) -> bool {
        // TODO:  <30-05-23> //

        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Projection {
    pub projection_attributes: HashSet<String>,
}

impl Hash for Projection {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for val in self.projection_attributes.iter() {
            val.hash(state);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rename {
    pub rename_pairs: HashMap<String, String>,
}
impl Hash for Rename {
    fn hash<H: Hasher>(&self, state: &mut H) {
        hash_hashmap(&self.rename_pairs, state);
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Extend {
    pub extend_pairs: HashMap<String, Function>,
}

impl Hash for Extend {
    fn hash<H: Hasher>(&self, state: &mut H) {
        hash_hashmap(&self.extend_pairs, state);
    }
}

pub type RcExtendFunction = Rc<Function>;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash)]
#[serde(tag = "type")]
pub enum Function {
    Reference { value: String },
    Constant { value: String },
    Template { value: String },
    UriEncode { inner_function: RcExtendFunction },
    Iri { inner_function: RcExtendFunction },
    Literal { inner_function: RcExtendFunction },
    BlankNode { inner_function: RcExtendFunction },
    Upper { inner_function: RcExtendFunction },
    Lower { inner_function: RcExtendFunction },
}

// Post-mapping operators

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Serializer {
    pub template: String,
    pub options:  Option<HashMap<String, String>>,
    pub format:   DataFormat,
}

impl Hash for Serializer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.template.hash(state);
        if let Some(option_map) = self.options.as_ref() {
            hash_hashmap(option_map, state);
        }
        self.format.hash(state);
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Target {
    pub configuration: HashMap<String, String>,
    pub target_type:   IOType,
    pub data_format:   DataFormat,
}

impl Hash for Target {
    fn hash<H: Hasher>(&self, state: &mut H) {
        hash_hashmap(&self.configuration, state);
        self.target_type.hash(state);
        self.data_format.hash(state);
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash)]
pub enum IOType {
    File,
    Kafka,
    Websocket,
    MySQL,
    PostgreSQL,
    SPARQLEndpoint,
}
