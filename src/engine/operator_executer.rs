use std::borrow::Borrow;
use std::collections::VecDeque;

use operator::{Operator, RcOperator};

trait Executor {
    fn reverse_algebra(algebra: RcOperator) -> VecDeque<Operator> {
        let mut queue = VecDeque::new();
        let mut current_algebra = algebra; 

        loop {
            match current_algebra.borrow() {
                Operator::SourceOp(config) => {
                    queue.push_back(Operator::SourceOp(config.clone()));
                    break;
                },
                Operator::JoinOp { config, operators } => todo!(),
                Operator::ProjectOp { config, operator } => todo!(),
                Operator::ExtendOp { config, operator } => todo!(),
                Operator::RenameOp { config, operator } => todo!(),
                Operator::SerializerOp { config, operator } => todo!(),
                Operator::TargetOp { config, operator } => todo!(),
            }
        }

        queue
    }
}
