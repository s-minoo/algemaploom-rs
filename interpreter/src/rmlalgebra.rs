use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;
use operator::{Operator, Projection, RcOperator};
use regex::Regex;

use crate::rml_model::term_map::{
    self, GraphMap, PredicateObjectMap, SubjectMap, TermMapInfo, TriplesMap,
};
use crate::rml_model::Document;

pub fn translate_to_algebra(doc: Document) -> Vec<Operator> {
    for tm in doc.triples_maps {
        let source_op = translate_source_op(tm);
        let projection =  translate_projection_op(tm, source_op);
    }
    todo!()
}

pub fn translate_source_op(tm: TriplesMap) -> RcOperator {
    Operator::SourceOp(tm.logical_source.into()).into()
    
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
    let value = tm_info.term_value.to_string();
    match tm_info.term_map_type {
        term_map::TermMapType::Constant => HashSet::new(),
        term_map::TermMapType::Reference => vec![value].into_iter().collect(),
        term_map::TermMapType::Template => get_attributes_from_template(&value).into_iter().collect(),
    }
}


pub fn translate_projection_op(tm: TriplesMap, parent_op: RcOperator) -> RcOperator {
    
    let mut projection_attributes =  get_attributes_from_term_map(&tm.subject_map.tm_info); 
    let mut gm_attributes = tm.graph_map.map_or(HashSet::new(), |gm| get_attributes_from_term_map(&gm.tm_info));

    let p_attributes:HashSet<_> =  tm.po_maps
        .iter()
        .flat_map(|pom| {
            let om_attrs = pom.object_maps
                .iter()
                .flat_map(|om| get_attributes_from_term_map(&om.tm_info));
            let pm_attrs = pom.predicate_maps
                .iter()
                .flat_map(|pm| get_attributes_from_term_map(&pm.tm_info));

            om_attrs.chain(pm_attrs)
        })
        .collect(); 

    // Subject map's attributes alread added to projection_attributes hashset 
    projection_attributes.extend(p_attributes);
    projection_attributes.extend(gm_attributes);

    Operator::ProjectOp(
        Projection{
            projection_attributes,
        }, parent_op).into()
}

pub fn translate_extend_op(tm:TriplesMap, parent_op: RcOperator) -> RcOperator{
    todo!()
}


#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use sophia_term::RcTerm;

    use super::*;

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
        let identifier = RcTerm
        let tm_info = TermMapInfo {
            identifier:      "".try_into(),
            logical_targets: HashSet::new(),
            term_map_type:   todo!(),
            term_value:      todo!(),
            term_type:       todo!(),
        };
    }
}
