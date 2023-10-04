mod operators;
mod types;
mod util;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use interpreter::rml_model::term_map::{SubjectMap, TermMapInfo, TermMapType};
use interpreter::rml_model::{Document, PredicateObjectMap, TriplesMap};
use operator::{
    Extend, Fragmenter, Function, Operator, Projection, RcExtendFunction,
    Serializer, Source,
};
use plangenerator::error::PlanError;
use plangenerator::plan::{join, Init, Plan, Processed, RcRefCellPlan};
use sophia_api::term::TTerm;

use self::operators::serializer::{self, translate_serializer_op};
use self::types::Triples;
use self::util::generate_lt_tm_map_from_spo;
use crate::rmlalgebra::types::SearchMap;
use crate::rmlalgebra::util::{
    generate_logtarget_map, generate_lt_tm_map_from_doc, generate_variable_map,
};

fn partition_pom_join_nonjoin(
    poms: Vec<PredicateObjectMap>,
) -> (Vec<PredicateObjectMap>, Vec<PredicateObjectMap>) {
    let (mut ptm_poms, mut no_ptm_poms): (Vec<_>, Vec<_>) =
        poms.into_iter().partition(|pom| pom.contains_ptm());

    for pom in ptm_poms.iter_mut() {
        let (ptm_oms, no_ptm_oms): (Vec<_>, Vec<_>) = pom
            .object_maps
            .clone()
            .into_iter()
            .partition(|om| om.parent_tm.is_some());

        pom.object_maps = ptm_oms;
        if !no_ptm_oms.is_empty() {
            no_ptm_poms.push(PredicateObjectMap {
                predicate_maps: pom.predicate_maps.clone(),
                object_maps:    no_ptm_oms,
            });
        }
    }

    (ptm_poms, no_ptm_poms)
}

pub fn translate_to_algebra(doc: Document) -> Result<Plan<Init>, PlanError> {
    let mut plan = Plan::<()>::new();

    let tm_projected_pairs_res: Result<Vec<_>, PlanError> = doc
        .triples_maps
        .iter()
        .map(|tm| {
            let source_op = translate_source_op(&tm);
            let projection_op = translate_projection_op(&tm);
            let result = (
                tm,
                Rc::new(RefCell::new(
                    plan.source(source_op)
                        .apply(&projection_op, "Projection")?,
                )),
            );

            Ok(result)
        })
        .collect();

    // Search dictionaries instantiations
    let variable_map = generate_variable_map(&doc);
    let target_map = generate_logtarget_map(&doc);
    let lt_id_tm_group_map = generate_lt_tm_map_from_doc(&doc);
    let mut tm_projected_pairs = tm_projected_pairs_res?;
    let tm_rccellplan_map: HashMap<_, _> = tm_projected_pairs
        .clone()
        .into_iter()
        .map(|(tm, rccellplan)| (tm.identifier.clone(), (tm, rccellplan)))
        .collect();

    let search_map = SearchMap {
        tm_rccellplan_map,
        variable_map,
        target_map,
        lt_id_tm_group_map,
    };

    // Finish search dictionaries instantiations

    tm_projected_pairs.iter().try_for_each(|(tm, plan)| {
        let sm_ref = &tm.subject_map;
        let poms = tm.po_maps.clone();

        let (joined_poms, no_join_poms): (Vec<_>, Vec<_>) =
            partition_pom_join_nonjoin(poms);

        if !joined_poms.is_empty() {
            add_join_related_ops(&joined_poms, sm_ref, &search_map, plan)?;
        }

        if !no_join_poms.is_empty() {
            add_non_join_related_ops(&no_join_poms, sm_ref, &search_map, plan)?;
        }

        Ok::<(), PlanError>(())
    })?;

    Ok(plan)
}

fn translate_fragment_op_from_lts_str(
    lt_triples_map: &HashMap<String, Vec<Triples>>,
    from_fragment: &str,
) -> Option<Fragmenter> {
    let target_lt_ids = lt_triples_map.keys();

    let to: Vec<String> = target_lt_ids.map(|id| id.clone()).collect();

    if to.len() == 1 && to.iter().next() == Some(&from_fragment.to_string()) {
        return None;
    }

    Some(Fragmenter {
        from: from_fragment.to_string(),
        to,
    })
}
fn translate_fragment_op_from_lts(
    lt_triples_map: &HashMap<String, Vec<Triples>>,
) -> Option<Fragmenter> {
    translate_fragment_op_from_lts_str(lt_triples_map, "default")
}

fn add_non_join_related_ops(
    no_join_poms: &[PredicateObjectMap],
    sm: &SubjectMap,
    search_map: &SearchMap,
    plan: &RcRefCellPlan<Processed>,
) -> Result<(), PlanError> {
    let variable_map = &search_map.variable_map;
    let target_map = &search_map.target_map;
    let extend_op = translate_extend_op(sm, no_join_poms, variable_map);
    let mut plan = plan.borrow_mut();
    let extended_plan = plan.apply(&extend_op, "ExtendOp")?;
    let mut next_plan = extended_plan;

    let lt_triples_map = generate_lt_tm_map_from_spo(sm, no_join_poms);
    let fragmenter = translate_fragment_op_from_lts(&lt_triples_map);
    let mut lt_id_vec = vec![lt_triples_map.keys().next().unwrap().clone()];

    println!("{:#?}", plan);
    if let Some(fragmenter) = fragmenter {
        next_plan = next_plan.fragment(fragmenter.clone())?;
        lt_id_vec = fragmenter.to;
    }

    for lt_id in lt_id_vec {
        // TODO: Fix target_map retrieval logic <29-09-23, yourname> //
        let target = target_map.get(&lt_id).unwrap();
        let serialize_format = &target.data_format;
        let triples = lt_triples_map.get(&lt_id).unwrap();

        let serializer_op = serializer::translate_serializer_op(
            triples,
            serialize_format,
            variable_map,
        );

        next_plan.serialize(serializer_op)?.sink(&target);

        //let _ = extended_plan.fragment(fragmenter)?.serialize(serializer_op);
    }

    Ok(())
}

fn add_join_related_ops(
    join_poms: &[PredicateObjectMap],
    sm: &SubjectMap,
    search_map: &SearchMap,
    plan: &RcRefCellPlan<Processed>,
) -> Result<(), PlanError> {
    // HashMap pairing the attribute with the function generated from
    // PTM's subject map

    let search_tm_plan_map = &search_map.tm_rccellplan_map;
    let variable_map = &search_map.variable_map;
    let lt_target_map = &search_map.target_map;

    for pom in join_poms {
        let pms = &pom.predicate_maps;
        let oms = &pom.object_maps;

        for (_om_idx, om) in oms.iter().enumerate() {
            let ptm_iri = om
                .parent_tm
                .as_ref()
                .ok_or(PlanError::GenericError(format!(
                    "Parent triples map needs to be present in OM: {:#?}",
                    om
                )))?
                .to_string();

            let (ptm, other_plan) = search_tm_plan_map.get(&ptm_iri).ok_or(
                PlanError::GenericError(format!(
                    "Parent triples map IRI is wrong: {}",
                    &ptm_iri
                )),
            )?;

            let join_cond = om.join_condition.as_ref().unwrap();
            let child_attributes = &join_cond.child_attributes;
            let parent_attributes = &join_cond.parent_attributes;
            let ptm_variable = variable_map.get(&ptm.identifier).unwrap();
            let ptm_alias =
                format!("join_{}", &ptm_variable[ptm_variable.len() - 1..]);

            let mut joined_plan = join(Rc::clone(plan), Rc::clone(other_plan))?
                .alias(&ptm_alias)?
                .where_by(child_attributes.clone())?
                .compared_to(parent_attributes.clone())?;

            // Prefix the attributes in the subject map with the alias of the PTM
            let mut ptm_sm_info = ptm.subject_map.tm_info.clone();

            ptm_sm_info.prefix_attributes(&ptm_alias);

            // Pair the ptm subject iri function with an extended attribute
            let ptm_sub_function =
                extract_extend_function_from_term_map(&ptm_sm_info);
            let om_extend_attr =
                variable_map.get(&om.tm_info.identifier).unwrap().clone();

            let pom_with_joined_ptm = vec![PredicateObjectMap {
                predicate_maps: pms.clone(),
                object_maps:    [om.clone()].to_vec(),
            }];

            let mut extend_pairs =
                translate_extend_pairs(variable_map, sm, &pom_with_joined_ptm);

            extend_pairs.insert(om_extend_attr, ptm_sub_function);

            let extend_op = Operator::ExtendOp {
                config: Extend { extend_pairs },
            };
            let mut extended_plan = joined_plan.apply(&extend_op, "Extend")?;

            let lt_triples_map =
                generate_lt_tm_map_from_spo(sm, &pom_with_joined_ptm);

            for lt_id in lt_triples_map.keys() {
                let triples = lt_triples_map.get(lt_id).unwrap();
                let target = lt_target_map.get(lt_id).unwrap();
                let serializer_op = translate_serializer_op(
                    triples,
                    &target.data_format,
                    variable_map,
                );

                extended_plan.serialize(serializer_op)?.sink(target)?;
            }
            //.serialize(serializer_op)?;
            //.sink(file_target(count));
        }
    }

    Ok(())
}
fn translate_source_op(tm: &TriplesMap) -> Source {
    tm.logical_source.clone().into()
}

fn translate_projection_op(tm: &TriplesMap) -> Operator {
    let mut projection_attributes = tm.subject_map.tm_info.get_attributes();
    let gm_attributes = tm
        .graph_map
        .clone()
        .map_or(HashSet::new(), |gm| gm.tm_info.get_attributes());

    let p_attributes: HashSet<_> = tm
        .po_maps
        .iter()
        .flat_map(|pom| {
            let om_attrs = pom.object_maps.iter().flat_map(|om| {
                if let Some(join_cond) = &om.join_condition {
                    let mut child_attr = join_cond.child_attributes.clone();
                    let mut parent_attr = join_cond.parent_attributes.clone();
                    child_attr.append(&mut parent_attr);
                    child_attr.into_iter().collect()
                } else {
                    om.tm_info.get_attributes()
                }
            });
            let pm_attrs = pom
                .predicate_maps
                .iter()
                .flat_map(|pm| pm.tm_info.get_attributes());

            om_attrs.chain(pm_attrs)
        })
        .collect();

    // Subject map's attributes alread added to projection_attributes hashset
    projection_attributes.extend(p_attributes);
    projection_attributes.extend(gm_attributes);

    Operator::ProjectOp {
        config: Projection {
            projection_attributes,
        },
    }
}

fn extract_extend_function_from_term_map(tm_info: &TermMapInfo) -> Function {
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

fn translate_extend_op(
    sm: &SubjectMap,
    poms: &[PredicateObjectMap],
    variable_map: &HashMap<String, String>,
) -> Operator {
    let extend_pairs = translate_extend_pairs(variable_map, sm, poms);

    operator::Operator::ExtendOp {
        config: Extend { extend_pairs },
    }
}

fn translate_extend_pairs(
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

fn sm_extract_extend_pair(
    variable_map: &HashMap<String, String>,
    sm: &SubjectMap,
) -> Vec<(String, Function)> {
    let sub_extend = vec![(
        variable_map.get(&sm.tm_info.identifier).unwrap().clone(),
        extract_extend_function_from_term_map(&sm.tm_info),
    )];
    sub_extend
}

fn extract_serializer_template<'a>(
    poms: &[PredicateObjectMap],
    sm: &SubjectMap,
    variable_map: &HashMap<String, String>,
) -> String {
    let subject = variable_map.get(&sm.tm_info.identifier).unwrap().clone();
    let predicate_objects = poms.iter().flat_map(|pom| {
        let _p_length = pom.predicate_maps.len();
        let _o_length = pom.object_maps.len();

        let predicates = pom
            .predicate_maps
            .iter()
            .flat_map(|pm| variable_map.get(&pm.tm_info.identifier));
        let objects = pom
            .object_maps
            .iter()
            .flat_map(|om| variable_map.get(&om.tm_info.identifier));

        predicates.flat_map(move |p_string| {
            objects
                .clone()
                .map(move |o_string| (p_string.clone(), o_string.clone()))
        })
    });

    predicate_objects
        .map(|(predicate, object)| {
            format!(" ?{} ?{} ?{}.", subject, predicate, object)
        })
        .fold(String::new(), |a, b| a + &b + "\n")
}

fn translate_serializer_op_old<'a>(
    poms: &[PredicateObjectMap],
    sm: &SubjectMap,
    variable_map: &HashMap<String, String>,
) -> Serializer {
    let template = extract_serializer_template(poms, sm, variable_map);
    Serializer {
        template,
        options: None,
        format: operator::formats::DataFormat::NTriples,
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;
    use std::collections::HashSet;

    use interpreter::extractors::io::parse_file;
    use interpreter::extractors::triplesmap_extractor::extract_triples_maps;
    use interpreter::rml_model::term_map;
    use sophia_term::Term;

    use super::*;
    use crate::import_test_mods;

    import_test_mods!();

    #[test]
    fn test_get_attributes_term_map_info() {
        let identifier = "tm_1".to_string();
        let template_term_map_info = TermMapInfo {
            identifier,
            logical_targets: HashSet::new(),
            term_map_type: term_map::TermMapType::Template,
            term_value: new_term_value("{id}{firstname}{lastname}".to_string()),
            term_type: None,
            fun_map_opt: None,
        };

        let attributes = template_term_map_info.get_attributes();
        let check = new_hash_set(["id", "firstname", "lastname"].into());

        assert_eq!(attributes, check);

        let reference_term_map_info = TermMapInfo {
            term_map_type: term_map::TermMapType::Reference,
            term_value: new_term_value("aReferenceValue".to_string()),
            ..template_term_map_info
        };

        let attributes = reference_term_map_info.get_attributes();
        let check = new_hash_set(["aReferenceValue"].into());
        assert_eq!(attributes, check);
    }

    #[test]
    fn test_projection_operator() -> ExtractorResult<()> {
        let graph = load_graph!("sample_mapping.ttl").unwrap();
        let mut triples_map_vec = extract_triples_maps(&graph)?;
        assert_eq!(triples_map_vec.len(), 1);

        let triples_map = triples_map_vec.pop().unwrap();
        let _source_op = translate_source_op(&triples_map);
        let projection_ops = translate_projection_op(&triples_map);

        let projection = match projection_ops.borrow() {
            Operator::ProjectOp { config: proj } => proj,
            _ => panic!("Parsed wrong! Operator should be projection"),
        };

        let check_attributes =
            new_hash_set(["stop", "id", "latitude", "longitude"].to_vec());

        assert_eq!(projection.projection_attributes, check_attributes);

        Ok(())
    }

    fn new_term_value(value: String) -> Term<String> {
        Term::new_literal_dt_unchecked(value, Term::new_iri("string").unwrap())
    }

    fn new_hash_set(v: Vec<&str>) -> HashSet<String> {
        v.into_iter().map(|st| st.to_string()).collect()
    }

    #[test]
    fn test_extend_operator() -> ExtractorResult<()> {
        let graph = load_graph!("sample_mapping.ttl").unwrap();
        let mut triples_map_vec = extract_triples_maps(&graph)?;
        assert_eq!(triples_map_vec.len(), 1);
        let triples_map = triples_map_vec.pop().unwrap();
        let _source_op = translate_source_op(&triples_map);
        let _projection_ops = translate_projection_op(&triples_map);

        let variable_map = generate_variable_map(&Document {
            triples_maps: triples_map_vec,
        });

        let extend_op = translate_extend_op(
            &triples_map.subject_map,
            &triples_map.po_maps,
            &variable_map,
        );

        println!("{:#?}", extend_op);
        Ok(())
    }

    #[test]
    fn test_operator_translation() -> ExtractorResult<()> {
        let document = parse_file(test_case!("sample_mapping.ttl").into())?;
        let operators = translate_to_algebra(document);

        let _output = File::create("op_trans_output.json")?;
        println!("{:#?}", operators);
        Ok(())
    }

    #[test]
    fn test_operator_translation_complex() -> ExtractorResult<()> {
        let document = parse_file(test_case!("multiple_tm.ttl").into())?;
        let operators = translate_to_algebra(document);

        let _output = File::create("op_trans_complex_output.json")?;
        println!("{:#?}", operators);
        Ok(())
    }
}
