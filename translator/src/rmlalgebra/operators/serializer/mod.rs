mod nquads;
mod ntriples;
mod util;

use std::collections::HashMap;

use operator::formats::DataFormat;
use operator::Serializer;

use self::nquads::NQuadsSerializer;
use self::ntriples::NTriplesSerializer;
use crate::rmlalgebra::types::Quads;

trait SerializeTranslator {
    fn translate(
        quads: &[Quads],
        variable_map: &HashMap<String, String>,
    ) -> Serializer;
}

pub fn translate_serializer_op(
    quads: &[Quads],
    serialize_format: &DataFormat,
    variable_map: &HashMap<String, String>,
) -> Serializer {
    match serialize_format {
        DataFormat::NTriples => {
            NTriplesSerializer::translate(quads, variable_map)
        }
        DataFormat::NQuads => NQuadsSerializer::translate(quads, variable_map),
        DataFormat::SQL => unimplemented!(),
        DataFormat::JSONLD => unimplemented!(),
        DataFormat::JSON => unimplemented!(),
        DataFormat::XML => unimplemented!(),
        DataFormat::CSV => unimplemented!(),
        DataFormat::TTL => unimplemented!(),
    }
}
