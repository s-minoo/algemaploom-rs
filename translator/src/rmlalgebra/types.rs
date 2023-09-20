use std::collections::HashMap;

use interpreter::rml_model::source_target::LogicalTarget;
use interpreter::rml_model::term_map::{
    GraphMap, ObjectMap, PredicateMap, SubjectMap,
};
use interpreter::rml_model::TriplesMap;
use plangenerator::plan::{Plan, Processed};








#[derive(Debug, Clone)]
pub enum TermMapEnum {
    SubjectMapEnum(SubjectMap),
    PredicateMapEnum(PredicateMap),
    ObjectMapEnum(ObjectMap),
    GraphMapEnum(GraphMap),
}
