use operator::tuples::SolutionMapping;
use operator::value::Value;

use super::{BoxedOperatorChainOpt, OperatorChain};

pub type ExtendFunction = Box<dyn Fn(&SolutionMapping) -> Vec<Value>>;

pub struct ExtendTuple {
    pub new_attribute: String,
    pub function:      ExtendFunction,
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
                (ext_pair.function)(mapping),
            );
        });
        todo!()
    }
}
