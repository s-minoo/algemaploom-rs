use std::borrow::Cow;
use std::collections::HashSet;

use lazy_static::lazy_static;
use operator::value::Value;
use regex::Regex;

use super::{BoxedFunctionChainOpt, FunctionChain};

lazy_static! {
    static ref TEMPLATE_REGEX: Regex = Regex::new("{(?<attr>[^}{]+)}").unwrap();
}

pub struct Template {
    pub template:         String,
    pub next:             BoxedFunctionChainOpt,
    attributes_regex_set: Vec<(String, Regex)>,
}

impl Template {
    pub fn new(template: String, next: BoxedFunctionChainOpt) -> Self {
        let attributes: HashSet<_> = TEMPLATE_REGEX
            .captures_iter(&template)
            .map(|cap| cap["attr"].to_string())
            .collect();

        let regex_iter = attributes.iter().map(|attr| {
            let pattern = format!("[^{{]*{{{}}}[^}}]*", attr);
            Regex::new(&pattern).unwrap()
        });

        let attributes_regex_set =
            attributes.clone().into_iter().zip(regex_iter).collect();

        Template {
            template,
            next,
            attributes_regex_set,
        }
    }
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

    fn process(&self, mapping: &operator::tuples::SolutionMapping) -> Value {
        let mut result = self.template.to_owned();
        for (attr, regex) in self.attributes_regex_set.iter() {
            let val = mapping.get(attr).unwrap().to_string();
            if let Cow::Owned(new) = regex.replace_all(&result, val) {
                result = new;
            }
        }

        Value::String(result)
    }

    fn process_value(&self, value: &Value) -> Value {
        panic!("Template function cannot process on Value input!")
    }
}
