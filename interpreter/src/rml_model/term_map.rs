use std::collections::HashSet;

use lazy_static::lazy_static;
use regex::Regex;
use sophia_api::term::{TTerm, TermKind};
use sophia_term::{RcTerm, Term};

use super::join::JoinCondition;
use super::source_target::LogicalTarget;
use crate::{IriString, TermString};

#[derive(Debug, Clone)]
pub struct TermMapInfo {
    pub identifier:      String,
    pub logical_targets: HashSet<LogicalTarget>,
    pub term_map_type:   TermMapType,
    pub term_value:      TermString,
    pub term_type:       Option<TermKind>,
}

impl Default for TermMapInfo {
    fn default() -> Self {
        Self {
            identifier:      Default::default(),
            logical_targets: Default::default(),
            term_map_type:   TermMapType::Constant,
            term_value:      Term::new_bnode("qsdkfldsfj").unwrap(),
            term_type:       Default::default(),
        }
    }
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

fn prefix_attributes_from_template(template: &str, prefix: &str) -> String {
    let sanitized = template.replace("\\{", "\\(").replace("\\}", "\\)");
    TEMPLATE_REGEX
        .replace_all(&sanitized, format!("{{{}_$1}}", prefix))
        .replace("\\(", "\\{")
        .replace("\\)", "\\}")
}

impl TermMapInfo {
    pub fn prefix_attributes(self, prefix: &str) -> Self {
        let term_value = match self.term_map_type {
            TermMapType::Constant => self.term_value,
            TermMapType::Reference => {
                self.term_value.map(|val| format!("{}_{}", prefix, val))
            }
            TermMapType::Template => {
                self.term_value
                    .map(|val| prefix_attributes_from_template(&val, prefix))
            }
        };

        TermMapInfo { term_value, ..self }
    }

    pub fn get_attributes(&self) -> HashSet<String> {
        let value = self.term_value.value().to_string();
        match self.term_map_type {
            TermMapType::Constant => HashSet::new(),
            TermMapType::Reference => vec![value].into_iter().collect(),
            TermMapType::Template => {
                get_attributes_from_template(&value).into_iter().collect()
            }
        }
    }
    pub fn from_constant_value(const_value: RcTerm) -> TermMapInfo {
        let identifier = match const_value.clone() {
            Term::Iri(iri) => Term::Iri(iri.map(|i| i.to_string())),
            Term::BNode(bnode) => Term::BNode(bnode.map(|i| i.to_string())),
            Term::Literal(lit) => {
                Term::new_bnode(format!(
                    "{}-{}",
                    lit.txt(),
                    uuid::Uuid::new_v4()
                ))
                .unwrap()
            }
            Term::Variable(_) => {
                panic!("Variable not supported yet!")
            }
        }
        .to_string();

        let term_type = Some(const_value.kind());

        TermMapInfo {
            identifier,
            logical_targets: HashSet::new(),
            term_map_type: TermMapType::Constant,
            term_value: const_value.map(|i| i.to_string()),
            term_type,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum TermMapType {
    Constant,
    Reference,
    Template,
}

#[derive(Debug, Clone)]
pub struct SubjectMap {
    pub tm_info: TermMapInfo,
    pub classes: Vec<IriString>,
}

#[derive(Debug, Clone)]
pub struct PredicateMap {
    pub tm_info: TermMapInfo,
}

#[derive(Debug, Clone)]
pub struct ObjectMap {
    pub tm_info:        TermMapInfo,
    pub parent_tm:      Option<IriString>,
    pub join_condition: Option<JoinCondition>,
    pub data_type:      Option<IriString>,
    pub language:       Option<String>,
}

#[derive(Debug, Clone)]
pub struct GraphMap {
    pub tm_info: TermMapInfo,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_replace_attribute() {
        let template = "http://example.com/{foo}/{bar}";
        let prefix = "alpha";
        let prefixed_template =
            prefix_attributes_from_template(template, prefix);

        assert_eq!(
            "http://example.com/alpha_foo/alpha_bar",
            &prefixed_template
        );
    }

    #[test]
    fn test_template_excape_replace() {
        let template = "http://example.com/\\{hello\\}/{foo}/{bar}";
        let prefix = "alpha";
        let prefixed_template =
            prefix_attributes_from_template(template, prefix);

        assert_eq!(
            "http://example.com/\\{hello\\}/alpha_foo/alpha_bar",
            &prefixed_template
        );
    }
}
