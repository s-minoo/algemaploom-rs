use operator::formats::DataFormat;

use super::SerializeTranslator;
use crate::rmlalgebra::operators::serializer::util::unterminated_triple_strings;

#[derive(Debug, Clone)]
pub struct NQuadsSerializer {}

impl SerializeTranslator for NQuadsSerializer {
    fn translate(
        quads: &[crate::rmlalgebra::types::Quads],
        variable_map: &std::collections::HashMap<String, String>,
    ) -> operator::Serializer {
        let mut quad_strings: Vec<String> = vec![];
        for quad in quads {
            let unterminated_triples =
                unterminated_triple_strings(quad, variable_map).into_iter();

            if quad.gms.is_empty() {
                quad_strings
                    .extend(unterminated_triples.map(|trip| trip + " ."));
            } else {
                for gm in &quad.gms {
                    let gm_var =
                        variable_map.get(&gm.tm_info.identifier).unwrap();

                    let quads_with_gm = unterminated_triples
                        .clone()
                        .map(|trip| format!("{} {} .", trip, gm_var));
                    quad_strings.extend(quads_with_gm);
                }
            }
        }

        let template = quad_strings.join("\n");

        operator::Serializer {
            template,
            options: None,
            format: DataFormat::NQuads,
        }
    }
}
