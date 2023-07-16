use urlencoding::encode;

use super::{BoxedFunctionChainOpt, FunctionChain};

pub struct URIEncode {
    pub next: BoxedFunctionChainOpt,
}

impl FunctionChain for URIEncode {
    fn next(&self) -> &BoxedFunctionChainOpt {
        &self.next
    }

    fn process(
        &self,
        _mapping: &operator::tuples::SolutionMapping,
    ) -> operator::value::Value {
        panic!("URIEncode cannot work on solution mappings");
    }

    fn process_value(
        &self,
        value: &operator::value::Value,
    ) -> operator::value::Value {
        let str: String = value.into();
        let encoded = encode(&str);
        encoded.to_string().into()
    }
    fn into_boxed_opt(self) -> BoxedFunctionChainOpt {
        Some(self.into_boxed())
    }

    fn into_boxed(self) -> super::BoxedFunctionChain {
        Box::new(self)
    }
}
