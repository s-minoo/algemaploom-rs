pub mod rdf_serializer;

use operator::tuples::MappingTuple;

pub trait Serializer {
    fn template(&self) -> &str;
    fn serialize(&self, tuple: MappingTuple) -> String;
}
