use operator::tuples::{MappingTuple, SolutionMapping, SolutionSequence};

pub mod extend;
pub mod project;
pub mod serializers;
pub mod sources;

type BoxedOperatorChainOpt = Option<Box<dyn OperatorChain>>;

pub trait OperatorChain {
    fn into_boxed_opt(self) -> BoxedOperatorChainOpt;
    fn next(&mut self) -> &mut BoxedOperatorChainOpt;

    fn process(&mut self, tuple: &mut MappingTuple) {
        tuple.iter_mut().for_each(|(_fragment, sequence)| {
            self.process_solution_sequence(sequence)
        });
    }

    fn process_solution_sequence(&mut self, sequence: &mut SolutionSequence) {
        sequence
            .iter_mut()
            .for_each(|mapping| self.process_solution_mapping(mapping));
    }

    fn process_solution_mapping(&mut self, mapping: &mut SolutionMapping);
    fn execute(&mut self, tuple: &mut MappingTuple) {
        self.process(tuple);

        if let Some(next_op) = self.next() {
            next_op.execute(tuple);
        }
    }
}
