use std::collections::{HashMap, HashSet};
use std::fmt::format;

use lazy_static::lazy_static;
use operator::{
    Extend, Function, Operator, Projection, RcExtendFunction, RcOperator,
    Serializer,
};
use regex::Regex;
use sophia_api::term::TTerm;

use crate::rml_model::term_map::{self, TermMapInfo, TermMapType, TriplesMap};
use crate::rml_model::Document;

pub fn translate_to_algebra(doc: Document) -> Vec<RcOperator> {
    doc.triples_maps
        .iter()
        .enumerate()
        .map(|(count, tm)| {
            let source_op = translate_source_op(&tm);
            let projection_op = translate_projection_op(&tm, source_op);
            let prefix_id = format!("?tm{}", count);
            let extend_op = translate_extend_op(&tm, projection_op, &prefix_id);
            let serializer_op =
                translate_serializer_op(&tm, extend_op, &prefix_id);

            serializer_op
        })
        .collect()
}

fn translate_source_op(tm: &TriplesMap) -> RcOperator {
    Operator::SourceOp(tm.logical_source.clone().into()).into()
}

lazy_static! {
    static ref TEMPLATE_REGEX: Regex = Regex::new(r"\{([^\{\}]+)\}").unwrap();
}

fn get_attributes_from_template(template: &str) -> Vec<String> {
    let sanitized = template.replace("\\{", "").replace("\\}", "");
    let captured = TEMPLATE_REGEX.captures_iter(&sanitized);
    captured
        .filter_map(|cap| cap.get(1).map(|c| c.as_str().to_owned()))
        .collect()
}

fn get_attributes_from_term_map(tm_info: &TermMapInfo) -> HashSet<String> {
    let value = tm_info.term_value.value().to_string();
    match tm_info.term_map_type {
        term_map::TermMapType::Constant => HashSet::new(),
        term_map::TermMapType::Reference => vec![value].into_iter().collect(),
        term_map::TermMapType::Template => {
            get_attributes_from_template(&value).into_iter().collect()
        }
    }
}

fn translate_projection_op(
    tm: &TriplesMap,
    parent_op: RcOperator,
) -> RcOperator {
    let mut projection_attributes =
        get_attributes_from_term_map(&tm.subject_map.tm_info);
    let gm_attributes = tm.graph_map.clone().map_or(HashSet::new(), |gm| {
        get_attributes_from_term_map(&gm.tm_info)
    });

    let p_attributes: HashSet<_> = tm
        .po_maps
        .iter()
        .flat_map(|pom| {
            let om_attrs = pom
                .object_maps
                .iter()
                .flat_map(|om| get_attributes_from_term_map(&om.tm_info));
            let pm_attrs = pom
                .predicate_maps
                .iter()
                .flat_map(|pm| get_attributes_from_term_map(&pm.tm_info));

            om_attrs.chain(pm_attrs)
        })
        .collect();

    // Subject map's attributes alread added to projection_attributes hashset
    projection_attributes.extend(p_attributes);
    projection_attributes.extend(gm_attributes);

    Operator::ProjectOp {
        config:   Projection {
            projection_attributes,
        },
        operator: parent_op,
    }
    .into()
}

fn extract_extend_function_from_term_map(
    tm_info: &TermMapInfo,
    attribute: String,
) -> (String, Function) {
    let term_value = tm_info.term_value.value().to_string();
    let value_function: RcExtendFunction = match tm_info.term_map_type {
        TermMapType::Constant => Function::Constant { value: term_value },
        TermMapType::Reference => Function::Reference { value: term_value },
        TermMapType::Template => Function::Template { value: term_value },
    }
    .into();

    let type_function = match tm_info.term_type.unwrap() {
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

    (attribute, type_function)
}

fn translate_extend_op(
    tm: &TriplesMap,
    parent_op: RcOperator,
    prefix_id: &str,
) -> RcOperator {
    let sub_extend = vec![extract_extend_function_from_term_map(
        &tm.subject_map.tm_info,
        format!("{}_sm", prefix_id),
    )];

    let poms_extend =
        tm.po_maps.iter().enumerate().flat_map(|(pom_count, pom)| {
            let predicate_extends = pom.predicate_maps.iter().enumerate().map(
                move |(p_count, pm)| {
                    extract_extend_function_from_term_map(
                        &pm.tm_info,
                        format!("{}_p{}-{}", prefix_id, pom_count, p_count),
                    )
                },
            );

            let object_extends =
                pom.object_maps
                    .iter()
                    .enumerate()
                    .map(move |(o_count, om)| {
                        extract_extend_function_from_term_map(
                            &om.tm_info,
                            format!("{}_o{}-{}", prefix_id, pom_count, o_count),
                        )
                    });
            predicate_extends.chain(object_extends)
        });

    let extend_ops_map: HashMap<String, Function> =
        poms_extend.chain(sub_extend).collect();

    operator::Operator::ExtendOp {
        config:   Extend {
            extend_pairs: extend_ops_map,
        },
        operator: parent_op,
    }
    .into()
}

fn extract_serializer_template(tm: &TriplesMap, prefix_id: &str) -> String {
    let subject = format!("{}_sm", prefix_id);
    let predicate_objects =
        tm.po_maps.iter().enumerate().flat_map(|(idx, pom)| {
            let p_length = pom.predicate_maps.len();
            let o_length = pom.object_maps.len();

            let predicates = (0..p_length).map(move |p_count| {
                format!("{}_p{}-{}", prefix_id, idx, p_count)
            });
            let objects = (0..o_length).map(move |o_count| {
                format!("{}_o{}-{}", prefix_id, idx, o_count)
            });

            let pairs = predicates.flat_map(move |p_string| {
                objects
                    .clone()
                    .map(move |o_string| (p_string.clone(), o_string.clone()))
            });

            pairs
        });

    let triple_graph_pattern = predicate_objects
        .map(|(predicate, object)| {
            format!(" {} {} {}.", subject, predicate, object)
        })
        .fold(String::new(), |a, b| a + &b + "\n");

    triple_graph_pattern
}

fn translate_serializer_op(
    tm: &TriplesMap,
    parent_op: RcOperator,
    prefix_id: &str,
) -> RcOperator {
    let template = extract_serializer_template(tm, prefix_id);
    Operator::SerializerOp {
        config:   Serializer {
            template,
            options: None,
            format: operator::formats::DataFormat::NT,
        },
        operator: parent_op,
    }
    .into()
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;
    use std::collections::HashSet;

    use sophia_term::Term;

    use super::*;
    use crate::extractors::io::parse_file;
    use crate::extractors::triplesmap_extractor::extract_triples_maps;
    use crate::import_test_mods;
    import_test_mods!();

    #[test]
    fn test_simple_template_regex() {
        let template = "http://www.example.com/{id}/{name}";
        let attributes = get_attributes_from_template(template);
        let check = vec!["id", "name"];

        assert_eq!(attributes, check);
    }

    #[test]
    fn test_escaped_template_regex() {
        let template = "http://www.example.com/\\{id\\}/{name}";
        let attributes = get_attributes_from_template(template);
        let check = vec!["name"];
        assert_eq!(attributes, check);
    }

    #[test]
    fn test_get_attributes_term_map_info() {
        let identifier = Term::new_iri("tm_1".to_owned()).unwrap();
        let template_term_map_info = TermMapInfo {
            identifier,
            logical_targets: HashSet::new(),
            term_map_type: term_map::TermMapType::Template,
            term_value: new_term_value("{id}{firstname}{lastname}".to_string()),
            term_type: None,
        };

        let attributes = get_attributes_from_term_map(&template_term_map_info);
        let check = new_hash_set(["id", "firstname", "lastname"].into());

        assert_eq!(attributes, check);

        let reference_term_map_info = TermMapInfo {
            term_map_type: term_map::TermMapType::Reference,
            term_value: new_term_value("aReferenceValue".to_string()),
            ..template_term_map_info
        };

        let attributes = get_attributes_from_term_map(&reference_term_map_info);
        let check = new_hash_set(["aReferenceValue"].into());
        assert_eq!(attributes, check);
    }

    #[test]
    fn test_projection_operator() -> ExtractorResult<()> {
        let graph = load_graph!("sample_mapping.ttl").unwrap();
        let mut triples_map_vec = extract_triples_maps(&graph)?;
        assert_eq!(triples_map_vec.len(), 1);

        let triples_map = triples_map_vec.pop().unwrap();
        let source_op = translate_source_op(&triples_map);
        let projection_ops =
            translate_projection_op(&triples_map, source_op.clone());

        let projection = match projection_ops.borrow() {
            Operator::ProjectOp {
                config: proj,
                operator: _,
            } => proj,
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
        let source_op = translate_source_op(&triples_map);
        let projection_ops =
            translate_projection_op(&triples_map, source_op.clone());

        let extend_op =
            translate_extend_op(&triples_map, projection_ops, "?tm1");

        println!("{:#?}", extend_op);
        Ok(())
    }

    #[test]
    fn test_operator_translation() -> ExtractorResult<()> {
        let document = parse_file(test_case!("sample_mapping.ttl").into())?;
        let operators = translate_to_algebra(document);

        let output = File::create("op_trans_output.json")?;
        serde_json::to_writer_pretty(output, &operators).unwrap();
        println!("{:#?}", operators);
        Ok(())
    }

    #[test]
    fn test_operator_translation_complex() -> ExtractorResult<()> {
        let document = parse_file(test_case!("multiple_tm.ttl").into())?;
        let operators = translate_to_algebra(document);

        let output = File::create("op_trans_complex_output.json")?;
        serde_json::to_writer_pretty(output, &operators).unwrap();
        println!("{:#?}", operators);
        Ok(())
    }
}
