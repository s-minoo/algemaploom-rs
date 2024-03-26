use std::collections::{HashMap, HashSet};

use operator::formats::DataFormat;

use super::util::unterminated_triple_strings;
use super::SerializeTranslator;
use crate::rmlalgebra::types::Quad;

#[derive(Debug, Clone)]
pub struct NTriplesSerializer {}

impl SerializeTranslator for NTriplesSerializer {
    fn translate(
        quads: &HashSet<Quad>,
        variable_map: &HashMap<String, String>,
    ) -> operator::Serializer {
        let mut triples_string: Vec<String> = Vec::new();
        for quad in quads {
            let terminated_triples =
                unterminated_triple_strings(quad, variable_map)
                    .into_iter()
                    .map(|str| format!("{} .", str));

            triples_string.extend(terminated_triples);
        }

        let template = triples_string.join("\n");

        operator::Serializer {
            template,
            options: None,
            format: DataFormat::NTriples,
        }
    }
}
