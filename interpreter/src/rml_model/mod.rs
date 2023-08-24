use self::source_target::LogicalSource;
use self::term_map::{GraphMap, ObjectMap, PredicateMap, SubjectMap};
use crate::TermString;

pub mod join;
pub mod source_target;
pub mod term_map;
pub mod expression_map;

pub struct Document {
    pub triples_maps: Vec<TriplesMap>,
}

#[derive(Debug, Clone)]
pub struct TriplesMap {
    pub identifier:     TermString,
    pub logical_source: LogicalSource,
    pub subject_map:    SubjectMap,
    pub po_maps:        Vec<PredicateObjectMap>,
    pub graph_map:      Option<GraphMap>,
}

#[derive(Debug, Clone)]
pub struct PredicateObjectMap {
    pub predicate_maps: Vec<PredicateMap>,
    pub object_maps:    Vec<ObjectMap>,
}
