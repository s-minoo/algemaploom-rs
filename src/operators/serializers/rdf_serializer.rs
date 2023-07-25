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
                for (attr, value) in solution_mapping.into_iter() {

                    let value: &str = value.into();
                    result =
                        result.replace(format!("?{}", attr).as_str(), value);
                }
            }
        }

        result
    }
}
