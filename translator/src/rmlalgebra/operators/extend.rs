use std::collections::HashMap;
use std::rc::Rc;

use interpreter::rml_model::term_map::{SubjectMap, TermMapInfo, TermMapType};
use interpreter::rml_model::PredicateObjectMap;
use operator::{Extend, Function, Operator, RcExtendFunction};
use sophia_api::term::TTerm;

pub fn extract_extend_function_from_term_map(
    tm_info: &TermMapInfo,
) -> Function {
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
                .map(|(param, om)| {
                    (
                        param.clone(),
                        Rc::new(extract_extend_function_from_term_map(
                            &om.tm_info,
                        )),
                    )
                })
                .collect();

            Function::FnO {
                fno_identifier,
                param_func_pairs,
            }
        }
    }
    .into();

    match tm_info.term_type.unwrap() {
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
    }
}

pub fn translate_extend_op(
    sm: &SubjectMap,
    poms: &[PredicateObjectMap],
    variable_map: &HashMap<String, String>,
) -> Operator {
    let extend_pairs = translate_extend_pairs(variable_map, sm, poms);

    operator::Operator::ExtendOp {
        config: Extend { extend_pairs },
    }
}

pub fn translate_extend_pairs(
    variable_map: &HashMap<String, String>,
    sm: &SubjectMap,
    poms: &[PredicateObjectMap],
) -> HashMap<String, Function> {
    let sub_extend = sm_extract_extend_pair(variable_map, sm);

    let poms_extend =
        poms.iter().flat_map(|pom| {
            let predicate_extends = pom.predicate_maps.iter().enumerate().map(
                move |(_p_count, pm)| {
                    (
                        variable_map
                            .get(&pm.tm_info.identifier)
                            .unwrap()
                            .clone(),
                        extract_extend_function_from_term_map(&pm.tm_info),
                    )
                },
            );

            let object_extends = pom.object_maps.iter().enumerate().map(
                move |(_o_count, om)| {
                    (
                        variable_map
                            .get(&om.tm_info.identifier)
                            .unwrap()
                            .clone(),
                        extract_extend_function_from_term_map(&om.tm_info),
                    )
                },
            );
            predicate_extends.chain(object_extends)
        });

    let extend_ops_map: HashMap<String, Function> =
        poms_extend.chain(sub_extend).collect();
    extend_ops_map
}

pub fn sm_extract_extend_pair(
    variable_map: &HashMap<String, String>,
    sm: &SubjectMap,
) -> Vec<(String, Function)> {
    let sub_extend = vec![(
        variable_map.get(&sm.tm_info.identifier).unwrap().clone(),
        extract_extend_function_from_term_map(&sm.tm_info),
    )];
    sub_extend
}
