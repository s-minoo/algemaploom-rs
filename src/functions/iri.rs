use operator::value::Value;

use super::{BoxedFunctionChainOpt, FunctionChain};

pub struct IRI {
    pub next: BoxedFunctionChainOpt,
}

impl FunctionChain for IRI {
    fn next(&self) -> &BoxedFunctionChainOpt {
        &self.next
    }

    fn process(
        &self,
        _mapping: &operator::tuples::SolutionMapping,
    ) -> operator::value::Value {
        panic!("IRI cannot work on solution mappings")
    }

    fn process_value(
        &self,
        value: &operator::value::Value,
    ) -> operator::value::Value {
        let mut str: String = value.into();
        str = format!("<{}>", str);

        let new_val = Value::String(str);

        if let Some(next_func) = &self.next {
            next_func.process_value(&new_val)
        } else {
            new_val
        }
    }

    fn into_boxed_opt(self) -> BoxedFunctionChainOpt {

        Some(self.into_boxed())
    }

    fn into_boxed(self) -> super::BoxedFunctionChain {
        Box::new(self)
    }
}
