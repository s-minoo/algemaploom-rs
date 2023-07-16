use operator::tuples::MappingTuple;

use super::Serializer;

pub struct RDFSerializer {
    pub template: String,
}

impl Serializer for RDFSerializer {
    fn template(&self) -> &str {
        &self.template
    }

    fn serialize(&self, tuple: MappingTuple) -> String {
        let mut result = self.template.clone();
        for (_fragment, solution_sequence) in tuple.into_iter() {
            for solution_mapping in solution_sequence.iter() {
                for (attr, values) in solution_mapping.into_iter() {
                    // TODO: Support value sequence serialization! <10-07-23, > //

                    let value: &str = values.first().unwrap().into();

                    result =
                        result.replace(format!("?{}", attr).as_str(), value);
                }
            }
        }

        result
    }
}
