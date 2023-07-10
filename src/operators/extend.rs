use super::{BoxedOperatorChainOpt, OperatorChain};
use crate::functions::BoxedFunctionChain;

pub struct ExtendTuple {
    pub new_attribute: String,
    pub function:      BoxedFunctionChain,
}

#[derive(Default)]
pub struct ExtendOp {
    extend_tuples: Vec<ExtendTuple>,
    next:          BoxedOperatorChainOpt,
}

impl OperatorChain for ExtendOp {
    fn next(&mut self) -> &mut BoxedOperatorChainOpt {
        &mut self.next
    }

    fn process_solution_mapping(
        &mut self,
        mapping: &mut operator::tuples::SolutionMapping,
    ) {
        self.extend_tuples.iter().for_each(|ext_pair| {
            mapping.insert(
                ext_pair.new_attribute.clone(),
                vec![ext_pair.function.process(mapping)],
            );
        });
        todo!()
    }
}
