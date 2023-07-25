use operator::{tuples::SolutionMapping, value::Value};


use crate::functions::BoxedFunctionChain;

use super::{BoxedOperatorChainOpt, OperatorChain};

pub struct ExtendTuple {
    pub new_attribute: String,
    pub function:      BoxedFunctionChain,
}

#[derive(Default)]
pub struct ExtendOp {
    pub extend_tuples: Vec<ExtendTuple>,
    pub next:          BoxedOperatorChainOpt,
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
                vec![ext_pair.function.process(mapping)],
            );
        });
        todo!()
    }

    fn process(&mut self, tuple: &mut operator::tuples::MappingTuple) {
        tuple.iter_mut().for_each(|(_fragment, sequence)| {
            self.process_solution_sequence(sequence)
        });
    }

    fn process_solution_sequence(
        &mut self,
        sequence: &mut operator::tuples::SolutionSequence,
    ) {
        sequence
            .iter_mut()
            .for_each(|mapping| self.process_solution_mapping(mapping));
    }

    fn execute(&mut self, tuple: &mut operator::tuples::MappingTuple) {
        self.process(tuple);

        if let Some(next_op) = self.next() {
            next_op.execute(tuple);
        }
    }

    fn into_boxed_opt(self) -> BoxedOperatorChainOpt {
        Some(Box::new(self))
    }
}
