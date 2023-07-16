use operator::{tuples::SolutionMapping, value::Value};


use super::{BoxedOperatorChainOpt, OperatorChain};

pub struct ExtendTuple {
    pub new_attribute: String,
    pub function:      Box<dyn FnMut(&SolutionMapping) -> Value> ,
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
        self.extend_tuples.iter_mut().for_each(|ext_pair| {
            mapping.insert(
                ext_pair.new_attribute.clone(),
                vec![(ext_pair.function)(mapping)],
            );
        });
        todo!()
    }
}
