use operator::value::Value;

use super::{BoxedFunctionChainOpt, FunctionChain};

pub struct Constant {
    pub constant: String,
    pub next:     BoxedFunctionChainOpt,
}

impl FunctionChain for Constant {
    fn next(&self) -> &BoxedFunctionChainOpt {
        &self.next
    }

    fn process(
        &self,
        _mapping: &operator::tuples::SolutionMapping,
    ) -> operator::value::Value {
        let value: Value = self.constant.clone().into();

        if let Some(next_func) = &self.next {
            next_func.process_value(&value)
        } else {
            value.to_owned()
        }
    }

    fn process_value(
        &self,
        _value: &operator::value::Value,
    ) -> operator::value::Value {
        self.constant.clone().into()
    }

    fn into_boxed(self) -> super::BoxedFunctionChain {
        Box::new(self)
    }

    fn into_boxed_opt(self) -> BoxedFunctionChainOpt {
        Some(self.into_boxed())
    }
}
