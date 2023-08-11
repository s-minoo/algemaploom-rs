use operator::tuples::{MappingTuple, SolutionMapping, SolutionSequence};
use operator::Operator;

use self::sources::to_physical_source;

pub mod extend;
pub mod project;
pub mod serializers;
pub mod sources;

pub type BoxedOperatorChain = Box<dyn OperatorChain>;
pub type BoxedOperatorChainOpt = Option<BoxedOperatorChain>;

pub trait OperatorChain {
    fn from_logical_operator(log_op: Operator) -> BoxedOperatorChainOpt
    where
        Self: Sized,
    {
        match log_op {
            Operator::SourceOp { config } => to_physical_source(config),
            Operator::JoinOp { config } => todo!(),
            Operator::ProjectOp { config } => todo!(),
            Operator::ExtendOp { config } => todo!(),
            Operator::RenameOp { config } => todo!(),
            Operator::SerializerOp { config } => todo!(),
            Operator::TargetOp { config } => todo!(),
        }
    }

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
