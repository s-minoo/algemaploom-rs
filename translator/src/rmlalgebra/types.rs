use std::collections::HashMap;

use interpreter::rml_model::source_target::LogicalTarget;
use interpreter::rml_model::term_map::{ObjectMap, PredicateMap, SubjectMap};
use interpreter::rml_model::{PredicateObjectMap, TriplesMap};
use plangenerator::plan::{Plan, Processed};

#[derive(Debug, Clone)]
pub struct RefPOM<'a> {
    pub pm: Vec<&'a PredicateMap>,
    pub om: Vec<&'a ObjectMap>,
}

impl<'a> From<&'a PredicateObjectMap> for RefPOM<'a> {
    fn from(value: &'a PredicateObjectMap) -> Self {
        Self {
            pm: value.predicate_maps.iter().collect(),
            om: value.object_maps.iter().collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Triples<'a> {
    pub sm:   &'a SubjectMap,
    pub poms: Vec<RefPOM<'a>>,
}

impl<'a> Triples<'a> {
    pub fn new(sm: &'a SubjectMap) -> Self {
        Self { sm, poms: vec![] }
    }
}

#[derive(Debug, Clone)]
pub struct SearchMap<'a> {
    pub tm_plan_map:        HashMap<String, (&'a TriplesMap, Plan<Processed>)>,
    pub variable_map:       HashMap<String, String>,
    pub logtarget_map:      HashMap<String, LogicalTarget>,
    pub lt_id_tm_group_map: HashMap<String, Vec<Triples<'a>>>,
}
