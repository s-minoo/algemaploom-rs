use std::collections::HashMap;

use operator::formats::DataFormat;

use super::SerializeTranslator;
use crate::rmlalgebra::types::Triples;

#[derive(Debug, Clone)]
pub struct NTriplesSerializer {}

impl SerializeTranslator for NTriplesSerializer {
    fn translate(
        triples_vec: &[Triples],
        variable_map: &HashMap<String, String>,
    ) -> operator::Serializer {
        let mut triples_string: Vec<String> = Vec::new();
        for triples in triples_vec {
            let sm = triples.sm;
            let sm_var = variable_map.get(&sm.tm_info.identifier).unwrap();

            for pom in &triples.poms {
                let p_os = pom.pm.iter().flat_map(|pm| {
                    let pm_var =
                        variable_map.get(&pm.tm_info.identifier).unwrap();

                    pom.om.iter().map(move |om| {
                        let om_var =
                            variable_map.get(&om.tm_info.identifier).unwrap();
                        format!("{} {}.", pm_var, om_var)
                    })
                });

                let s_p_os = p_os.map(|p_o| format!("{} {}", sm_var, p_o));
                triples_string.extend(s_p_os);
            }
        }

        let template = triples_string.join("\n");

        operator::Serializer {
            template,
            options: None,
            format: DataFormat::NTriples,
        }
    }
}

