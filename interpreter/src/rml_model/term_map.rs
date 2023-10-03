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

impl TermMapInfo {
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
    Function,
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
    pub fno_opt:        Option<FunctionMap>,
}

#[derive(Debug, Clone)]
pub struct FunctionMap {
    pub identifier:     String,
    pub function_iri:   String,
    pub param_om_pairs: Vec<(String, ObjectMap)>,
}

#[derive(Debug, Clone)]
pub struct GraphMap {
    pub tm_info: TermMapInfo,
}
