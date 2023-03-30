use std::collections::HashSet;

use super::join::JoinCondition;
use super::source_target::{LogicalSource, LogicalTarget};
use super::term::Term;

#[derive(Debug, Clone)]
pub struct TermMapInfo {
    pub identifier:      String,
    pub logical_targets: HashSet<LogicalTarget>,
    pub term_map_type:   TermMapType,
    pub term_value:      Term,
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
    pub tm_info: TermMapInfo,
    pub parent_tm: Option<String>,  
    pub join_condition: Option<JoinCondition>, 
}

#[derive(Debug, Clone)]
pub struct GraphMap {
    pub tm_info: TermMapInfo,
}
