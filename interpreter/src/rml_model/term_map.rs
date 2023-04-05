use std::collections::HashSet;

use sophia_api::term::{TTerm, TermKind};

use super::join::JoinCondition;
use super::source_target::{LogicalSource, LogicalTarget};
use crate::{IriString, TermString};

#[derive(Debug, Clone)]
pub struct TermMapInfo {
    pub identifier:      IriString,
    pub logical_targets: HashSet<LogicalTarget>,
    pub term_map_type:   TermMapType,
    pub term_value:      TermString,
    pub term_type:       Option<TermKind>,
}

#[derive(Debug, Clone)]
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

pub trait ConstantTermMapInfo<T> {
    fn constant_term_map(
        identifier: IriString,
        logical_targets: HashSet<LogicalTarget>,
        term_value: TermString,
    ) -> T;
}

impl ConstantTermMapInfo<TermMapInfo> for TermMapInfo {
    fn constant_term_map(
        identifier: IriString,
        logical_targets: HashSet<LogicalTarget>,
        term_value: TermString,
    ) -> TermMapInfo {
        let term_type = Some(term_value.kind());
        TermMapInfo {
            identifier,
            logical_targets,
            term_map_type: TermMapType::Constant,
            term_value,
            term_type,
        }
    }
}
