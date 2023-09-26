mod ntriples;

use std::collections::HashMap;

use operator::formats::DataFormat;
use operator::Serializer;

use self::ntriples::NTriplesSerializer;
use crate::rmlalgebra::types::Triples;

trait SerializeTranslator {
    fn translate(
        triples: &[Triples],
        variable_map: &HashMap<String, String>,
    ) -> Serializer;
}

pub fn translate_serializer_op(
    triples: &[Triples],
    serialize_format: &DataFormat,
    variable_map: &HashMap<String, String>,
) -> Serializer {
    match serialize_format {
        DataFormat::NTriples => {
            NTriplesSerializer::translate(triples, variable_map)
        }
        DataFormat::NQuads => unimplemented!(),
        DataFormat::SQL => unimplemented!(),
        DataFormat::JSONLD => unimplemented!(),
        DataFormat::JSON => unimplemented!(),
        DataFormat::XML => unimplemented!(),
        DataFormat::CSV => unimplemented!(),
        DataFormat::TTL => unimplemented!(),
    }
}
