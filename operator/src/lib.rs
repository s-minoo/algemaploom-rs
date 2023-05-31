pub mod formats;
mod test_util;
pub mod value;

use std::collections::HashMap;

use formats::DataFormat;
use value::{MapTypedValue, Value};

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    SourceOp(Source, Box<Operator>),
    TransformOp(Transform, Box<Operator>),
    JoinOp(Join, Vec<Operator>),
    MappingOp(Mapping, Box<Operator>),
    SerializerOp(Serializer, Box<Operator>),
    TargetOp(Target),
}

// Data items for communications
#[derive(Debug, Clone, PartialEq)]
pub struct DataItem {
    pub fields_value: HashMap<String, Value>,
}

// Pre-mapping operators

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

pub type ConditionExtractor = Box<dyn Fn(DataItem) -> bool>;
// Join operators
#[derive(Debug, Clone, PartialEq)]
pub struct Join {}
impl Join {
    pub fn is_binary_join(&self) -> bool {
        // TODO:  <30-05-23, Sitt Min Oo> //

        todo!()
    }
}

// Mapping operators

#[derive(Debug, Clone, PartialEq)]
pub struct Mapping {
    pub item_mappings: Vec<ItemMappingSpec>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ItemMappingSpec {
    pub map_attribute:  String,
    pub map_type_value: MapTypedValue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MappedDataItem {
    pub attr_vals_map: HashMap<String, Vec<Value>>,
}

// Post-mapping operators

// TODO: Unit struct for now since I have
// no idea which fields are required for the
// serializer component <26-04-23, Min Oo> //
#[derive(Debug, Clone, PartialEq)]
pub struct Serializer {}

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

        // TODO: FIX Mapping struct init <25-04-23, yourname> //
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
                    item_mappings: Vec::new(),
                },
                Box::new(Operator::TargetOp(Target {
                    configuration: HashMap::from([(
                        "path".into(),
                        "output.nt".into(),
                    )]),
                    target_type:   IOType::File,
                    data_format:   DataFormat::NT,
                })),
            )),
        );

        println!("{:#?}", chains);
    }
}
