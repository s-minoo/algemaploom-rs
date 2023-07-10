use std::collections::HashSet;

use super::{BoxedOperatorChainOpt, OperatorChain};

#[derive(Default)]
pub struct Projection {
    select_attributes: HashSet<String>,
    next:              BoxedOperatorChainOpt,
}

impl OperatorChain for Projection {
    fn next(&mut self) -> &mut BoxedOperatorChainOpt {
        &mut self.next
    }

    fn process_solution_mapping(
        &mut self,
        mapping: &mut operator::tuples::SolutionMapping,
    ) {
        mapping.retain(|k, _v| self.select_attributes.contains(k));
    }
}
