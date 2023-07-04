use operator::RcOperator;

trait Executor {
    fn execute(mapping_plan: RcOperator);
}
