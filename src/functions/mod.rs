use operator::tuples::SolutionMapping;
use operator::value::Value;

pub mod constant;
pub mod iri;
pub mod literal;
pub mod reference;
pub mod template;
pub mod uriencode;

pub type BoxedFunctionChainOpt = Option<Box<dyn FunctionChain>>;
pub type BoxedFunctionChain = Box<dyn FunctionChain>;

pub trait FunctionChain {
    fn into_boxed_opt(self) -> BoxedFunctionChainOpt;
    fn into_boxed(self) -> BoxedFunctionChain;

    fn next(&self) -> &BoxedFunctionChainOpt;

    fn process(&self, mapping: &SolutionMapping) -> Value;

    fn process_value(&self, value: &Value) -> Value;
}
