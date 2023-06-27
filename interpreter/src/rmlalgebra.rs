use std::collections::HashSet;

use lazy_static::lazy_static;
use operator::{Operator, Projection, RcOperator};
use regex::Regex;
use sophia_api::term::TTerm;

use crate::rml_model::term_map::{self, TermMapInfo, TriplesMap};
use crate::rml_model::Document;

pub fn translate_to_algebra(doc: Document) -> Vec<Operator> {
    for tm in doc.triples_maps {
        let source_op = translate_source_op(&tm);
        let projection = translate_projection_op(&tm, source_op);
    }
    todo!()
}

pub fn translate_source_op(tm: &TriplesMap) -> RcOperator {
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

pub fn translate_projection_op(
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

    Operator::ProjectOp(
        Projection {
            projection_attributes,
        },
        parent_op,
    )
    .into()
}

pub fn translate_extend_op(
    tm: TriplesMap,
    parent_op: RcOperator,
) -> RcOperator {
    todo!()
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;
    use std::collections::HashSet;

    use sophia_term::Term;

    use crate::extractors::triplesmap_extractor::extract_triples_maps;
    use crate::import_test_mods;

    use super::*;
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
            Operator::ProjectOp(proj, _source) => proj,
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
}
