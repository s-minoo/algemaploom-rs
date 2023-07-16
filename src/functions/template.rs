use lazy_static::lazy_static;
use regex::Regex;

use super::{BoxedFunctionChainOpt, FunctionChain};

// TODO: Proper regex replacement!  <12-07-23, > //

pub struct Template {
    pub template: String,
    pub next:     BoxedFunctionChainOpt,
}

lazy_static! {
    static ref TEMPLATE_REGEX: Regex = Regex::new("({[^}{]+})").unwrap();
}

impl FunctionChain for Template {
    fn into_boxed_opt(self) -> super::BoxedFunctionChainOpt {
        Some(self.into_boxed())
    }

    fn into_boxed(self) -> super::BoxedFunctionChain {
        Box::new(self)
    }

    fn next(&self) -> &super::BoxedFunctionChainOpt {
        &self.next
    }

    fn process(
        &self,
        mapping: &operator::tuples::SolutionMapping,
    ) -> operator::value::Value {
        todo!()
    }

    fn process_value(
        &self,
        value: &operator::value::Value,
    ) -> operator::value::Value {
        todo!()
    }
}
