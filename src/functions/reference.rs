use super::{BoxedFunctionChainOpt, FunctionChain};

pub struct Reference {
    next:      BoxedFunctionChainOpt,
    reference: String,
}

impl FunctionChain for Reference {
    fn next(&self) -> &BoxedFunctionChainOpt {
        &self.next
    }

    fn process(
        &self,
        mapping: &operator::tuples::SolutionMapping,
    ) -> operator::value::Value {
        let value = mapping
            .get(&self.reference)
            .unwrap()
            .first()
            .unwrap()
            .to_owned();

        if let Some(next_func) = &self.next {
            next_func.process_value(&value)
        } else {
            value
        }
    }

    fn process_value(
        &self,
        _value: &operator::value::Value,
    ) -> operator::value::Value {
        panic!("Reference function cannot process Value");
    }
    fn into_boxed_opt(self) -> BoxedFunctionChainOpt {

        Some(self.into_boxed())
    }

    fn into_boxed(self) -> super::BoxedFunctionChain {
        Box::new(self)
    }
}
