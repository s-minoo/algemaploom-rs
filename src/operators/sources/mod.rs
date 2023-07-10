use operator::Source as SourceConfiguration;

use super::OperatorChain;

pub mod file;

#[derive(Debug, Clone)]
pub struct Source {
    pub config: SourceConfiguration,
}


impl OperatorChain for Source{
    fn next(&mut self) -> &mut super::BoxedOperatorChainOpt {
        todo!()
    }

    fn process_solution_mapping(&mut self, mapping: &mut operator::tuples::SolutionMapping) {
        todo!()
    }
}
