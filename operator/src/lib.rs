pub mod formats;
mod test_util;
pub mod tuples;
pub mod value;

use std::collections::HashMap;

use formats::DataFormat;

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    SourceOp(Source, Box<Operator>),
    TransformOp(Transform, Box<Operator>),
    JoinOp(Join, Vec<Operator>),
    ProjectOp(Projection, Box<Operator>),
    ExtendOp(Extend, Box<Operator>),
    RenameOp(Rename, Box<Operator>),
    SerializerOp(Serializer, Box<Operator>),
    TargetOp(Target),
}

#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
pub enum Transform {
    ArbitraryTransform(FFIConfig),
    Lower,
    Upper,
}

////

// Join operators

#[derive(Debug, Clone, PartialEq)]
pub enum JoinType {
    LeftJoin,
    EquiJoin,
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct Projection {
    pub projection_attributes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rename {
    pub rename_pairs: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Extend {
    pub extend_pairs: HashMap<String, String>,
}

// Post-mapping operators

// TODO: Unit struct for now since I have
// no idea which fields are required for the
// serializer component <26-04-23, Min Oo> //
#[derive(Debug, Clone, PartialEq)]
pub struct Serializer {
    pub template: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Target {
    pub configuration: HashMap<String, String>,
    pub target_type:   IOType,
    pub data_format:   DataFormat,
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
    fn test_double_rml() {
        let file = test_resource!("join_mapping.ttl");
    }

    #[test]
    fn test_simple_rml() {
        let file = test_resource!("sample_mapping.ttl");
    }
}
