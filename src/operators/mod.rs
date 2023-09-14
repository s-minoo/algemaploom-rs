use anyhow::Result;
use operator::tuples::{MappingTuple, SolutionMapping, SolutionSequence};
use operator::Operator;

use self::sources::to_physical_source;
pub mod extend;
pub mod project;
pub mod serializers;
pub mod sources;

pub type BoxedOperatorChain = Box<dyn OperatorChain>;
pub type BoxedOperatorChainOpt = Option<BoxedOperatorChain>;

pub trait AsyncOperatorExecute {
    fn next(&mut self) -> BoxedOperatorChain;

    async fn execute(&mut self) -> Result<()> {
        Ok(())
    }
}

pub trait OperatorChain {
    fn from_logical_operator(log_op: Operator) -> BoxedOperatorChainOpt
    where
        Self: Sized,
    {
        match log_op {
            Operator::SourceOp { config } => to_physical_source(config),
            Operator::JoinOp { config: _ } => todo!(),
            Operator::ProjectOp { config: _ } => todo!(),
            Operator::ExtendOp { config: _ } => todo!(),
            Operator::RenameOp { config: _ } => todo!(),
            Operator::SerializerOp { config: _ } => todo!(),
            Operator::TargetOp { config: _ } => todo!(),
            Operator::FragmentOp { config: _ } => todo!(),
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
