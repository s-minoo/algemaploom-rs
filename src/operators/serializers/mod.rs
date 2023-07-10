use operator::tuples::MappingTuple;

pub trait Serializer {
    fn template(&self) -> String;
    fn serialize(&self, tuple: &MappingTuple) -> String;
}
