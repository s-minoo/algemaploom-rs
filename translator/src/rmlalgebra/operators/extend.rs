use std::collections::HashMap;


use rml_interpreter::rml_model::term_map::{
    SubjectMap, TermMapInfo, TermMapType,
};
use rml_interpreter::rml_model::PredicateObjectMap;
use operator::{Extend, Function, Operator, RcExtendFunction};
use sophia_api::term::TTerm;

use super::RMLTranslator;
use crate::rmlalgebra::util::{
    extract_gm_tm_infos, extract_tm_infos_from_poms,
};

#[derive(Debug, Clone)]
pub struct ExtendTranslator<'a> {
    pub tms:          Vec<&'a TermMapInfo>,
    pub variable_map: &'a HashMap<String, String>,
}

impl<'a> RMLTranslator<Operator> for ExtendTranslator<'a> {
    fn translate(self) -> Operator {
        let mut extend_pairs = HashMap::new();
        for tm_info in self.tms {
            let (variable, function) = extract_extend_function_from_term_map(
                self.variable_map,
                tm_info,
            );
            extend_pairs.insert(variable, function);
        }

        Operator::ExtendOp {
            config: Extend { extend_pairs },
        }
    }
}

pub fn extract_extend_function_from_term_map(
    variable_map: &HashMap<String, String>,
    tm_info: &TermMapInfo,
) -> (String, Function) {
    let term_value = tm_info.term_value.value().to_string();
    let value_function: RcExtendFunction = match tm_info.term_map_type {
        TermMapType::Constant => Function::Constant { value: term_value },
        TermMapType::Reference => Function::Reference { value: term_value },
        TermMapType::Template => Function::Template { value: term_value },
        TermMapType::Function => {
            let fn_map = tm_info.fun_map_opt.as_ref().unwrap();
            let fno_identifier = fn_map.function_iri.clone();
            let param_func_pairs = fn_map
                .param_om_pairs
                .iter()
                .map(|(_param, om)| {
                    extract_extend_function_from_term_map(
                        variable_map,
                        &om.tm_info,
                    )
                })
                .map(|(param, func)| (param, func.into()))
                .collect();

            Function::FnO {
                fno_identifier,
                param_func_pairs,
            }
        }
    }
    .into();

    let func = match tm_info.term_type.unwrap() {
        sophia_api::term::TermKind::Iri => {
            Function::Iri {
                inner_function: Function::UriEncode {
                    inner_function: value_function,
                }
                .into(),
            }
        }
        sophia_api::term::TermKind::Literal => {
            Function::Literal {
                inner_function: value_function,
            }
        }
        sophia_api::term::TermKind::BlankNode => {
            Function::BlankNode {
                inner_function: value_function,
            }
        }
        typ => panic!("Unrecognized term kind {:?}", typ),
    };

    (
        variable_map.get(&tm_info.identifier).unwrap().to_string(),
        func,
    )
}

pub fn translate_extend_pairs(
    variable_map: &HashMap<String, String>,
    sm: &SubjectMap,
    poms: &[PredicateObjectMap],
) -> HashMap<String, Function> {
    let mut tms = extract_tm_infos_from_poms(poms);
    tms.push(&sm.tm_info);
    tms.extend(extract_gm_tm_infos(sm, poms));

    tms.into_iter()
        .map(|tm| extract_extend_function_from_term_map(variable_map, tm))
        .collect()
}
