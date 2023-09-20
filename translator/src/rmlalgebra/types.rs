use interpreter::rml_model::term_map::{SubjectMap, PredicateMap, ObjectMap, GraphMap};
use interpreter::rml_model::{PredicateObjectMap, TriplesMap};


pub type PrefixTriplesMap= (String, TriplesMap); 
pub type VariableSubjectMap = (String, SubjectMap); 
pub type PrefixPOM = (String, PredicateObjectMap); 
pub type VariablePM = (String, PredicateMap); 
pub type VariableOM = (String, ObjectMap);



pub enum TermMapEnum {
    SubjectMapEnum(SubjectMap), 
    PredicateMapEnum(PredicateMap),
    ObjectMapEnum(ObjectMap),
    GraphMapEnum(GraphMap)
}
