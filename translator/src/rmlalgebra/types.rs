use interpreter::rml_model::term_map::{SubjectMap, PredicateMap, ObjectMap};
use interpreter::rml_model::{PredicateObjectMap, TriplesMap};



pub type PrefixTriplesMap= (String, TriplesMap); 
pub type VariableSubjectMap = (String, SubjectMap); 
pub type PrefixPOM = (String, PredicateObjectMap); 
pub type VariablePM = (String, PredicateMap); 
pub type VariableOM = (String, ObjectMap);

