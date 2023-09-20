use self::source_target::LogicalSource;
use self::term_map::{GraphMap, ObjectMap, PredicateMap, SubjectMap};


pub mod join;
pub mod source_target;
pub mod term_map;

#[derive(Debug, Clone)]
pub struct Document {
    pub triples_maps: Vec<TriplesMap>,
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
pub struct PredicateObjectMap {
    pub predicate_maps: Vec<PredicateMap>,
    pub object_maps:    Vec<ObjectMap>,
}

impl PredicateObjectMap {
    pub fn contains_ptm(&self) -> bool {
        self.object_maps
            .iter().any(|om| om.parent_tm.is_some())
    }
}
