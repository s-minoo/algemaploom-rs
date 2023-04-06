use std::collections::HashSet;

use sophia_api::term::{TTerm, TermKind};
use sophia_term::{Term, RcTerm};

use super::join::JoinCondition;
use super::source_target::{LogicalSource, LogicalTarget};
use crate::{IriString, TermString};

#[derive(Debug, Clone)]
pub struct TermMapInfo {
    pub identifier:      TermString,
    pub logical_targets: HashSet<LogicalTarget>,
    pub term_map_type:   TermMapType,
    pub term_value:      TermString,
    pub term_type:       Option<TermKind>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TermMapType {
    Constant,
    Reference,
    Template,
}

#[derive(Debug, Clone)]
pub struct TriplesMap {
    pub identifier:     String,
    pub logical_source: LogicalSource,
    pub subject_map:    SubjectMap,
    pub po_maps:        Vec<PredicateObjectMap>,
    pub graph_map:      Option<GraphMap>,
}

#[derive(Debug, Clone)]
pub struct SubjectMap {
    pub tm_info: TermMapInfo,
    pub classes: Vec<IriString>,
}

#[derive(Debug, Clone)]
pub struct PredicateObjectMap {
    pub predicate_maps: Vec<PredicateMap>,
    pub object_maps:    Vec<ObjectMap>,
}

#[derive(Debug, Clone)]
pub struct PredicateMap {
    pub tm_info: TermMapInfo,
}

#[derive(Debug, Clone)]
pub struct ObjectMap {
    pub tm_info:        TermMapInfo,
    pub parent_tm:      Option<String>,
    pub join_condition: Option<JoinCondition>,
    pub data_type:      Option<IriString>,
    pub language:       Option<String>,
}

#[derive(Debug, Clone)]
pub struct GraphMap {
    pub tm_info: TermMapInfo,
}

impl TermMapInfo {
    pub fn from_constant_value(const_value: RcTerm) -> TermMapInfo {
        let identifier: TermString = match const_value.clone() {
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
        };
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
