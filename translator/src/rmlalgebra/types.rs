use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use operator::Target;
use plangenerator::plan::{Plan, Processed};
use rml_interpreter::rml_model::term_map::{
    GraphMap, ObjectMap, PredicateMap, SubjectMap,
};
use rml_interpreter::rml_model::{PredicateObjectMap, TriplesMap};

#[derive(Debug, Clone)]
pub struct RefPOM<'a> {
    pub pm: Vec<&'a PredicateMap>,
    pub om: Vec<&'a ObjectMap>,
}

impl<'a> PartialEq for RefPOM<'a> {
    fn eq(&self, other: &Self) -> bool {
        let pm_identifiers_left: Vec<_> =
            self.pm.iter().map(|pm| &pm.tm_info.identifier).collect();
        let pm_identifiers_right: Vec<_> =
            other.pm.iter().map(|pm| &pm.tm_info.identifier).collect();
        let om_identifiers_left: Vec<_> =
            self.om.iter().map(|om| &om.tm_info.identifier).collect();
        let om_identifiers_right: Vec<_> =
            other.om.iter().map(|om| &om.tm_info.identifier).collect();
        pm_identifiers_left == pm_identifiers_right
            && om_identifiers_left == om_identifiers_right
    }
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
pub struct Quads<'a> {
    pub triples: Triples<'a>,
    pub gms:     Vec<&'a GraphMap>,
}

impl<'a> PartialEq for Quads<'a> {
    fn eq(&self, other: &Self) -> bool {
        let gm_identifiers_left: Vec<_> =
            self.gms.iter().map(|gm| &gm.tm_info.identifier).collect();
        let gm_identifiers_right: Vec<_> =
            other.gms.iter().map(|gm| &gm.tm_info.identifier).collect();

        self.triples == other.triples
            && gm_identifiers_left == gm_identifiers_right
    }
}

#[derive(Debug, Clone)]
pub struct Triples<'a> {
    pub sm:   &'a SubjectMap,
    pub poms: Vec<RefPOM<'a>>,
}

impl<'a> PartialEq for Triples<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.sm.tm_info.identifier == other.sm.tm_info.identifier
            && self.poms == other.poms
    }
}

pub type TMPlanPair<'a> = (&'a TriplesMap, Rc<RefCell<Plan<Processed>>>);
#[derive(Debug, Clone)]
pub struct SearchMap<'a> {
    pub tm_rccellplan_map:  HashMap<String, TMPlanPair<'a>>,
    pub variable_map:       HashMap<String, String>,
    pub target_map:         HashMap<String, Target>,
    pub lt_id_tm_group_map: HashMap<String, Vec<Quads<'a>>>,
}
