use std::collections::HashSet;

use operator::formats::DataFormat;

use super::SerializeTranslator;
use crate::rmlalgebra::operators::serializer::util::unterminated_triple_strings;
use crate::rmlalgebra::types::Quad;

#[derive(Debug, Clone)]
pub struct NQuadsSerializer {}

impl SerializeTranslator for NQuadsSerializer {
    fn data_format() -> DataFormat {
        DataFormat::NQuads
    }

    fn generate_template(
        quads: &HashSet<Quad>,
        variable_map: &std::collections::HashMap<String, String>,
    ) -> HashSet<String> {
        let mut quad_strings: HashSet<String> = HashSet::new();
        for quad in quads {
            let unterminated_triples =
                unterminated_triple_strings(quad, variable_map).into_iter();

            if quad.gm_opt.is_none() {
                quad_strings
                    .extend(unterminated_triples.map(|trip| trip + " ."));
            } else {
                for gm in &quad.gm_opt {
                    let gm_var =
                        variable_map.get(&gm.tm_info.identifier).unwrap();

                    let quads_with_gm = unterminated_triples
                        .clone()
                        .map(|trip| format!("{} {} .", trip, gm_var));
                    quad_strings.extend(quads_with_gm);
                }
            }
        }
        quad_strings
    }
}
