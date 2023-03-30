use super::term::Term;


pub trait TermMap{

    fn identifier(&self) -> String;
    fn term_map_type(&self) -> TermMapType; 
    fn term_value(&self) -> Term; 
        
}



#[derive(Debug, Clone)]
pub enum TermMapType{
    Constant,
    Reference,
    Template
}



#[derive(Debug, Clone)]
pub struct TriplesMap {
    pub logical_source: LogicalSource,
    pub subject_map:    SubjectMap,
    pub po_maps:        Vec<PredicateObjectMap>,
    pub graph_map:      Option<GraphMap>,
}

#[derive(Debug, Clone)]
pub struct LogicalSource {}

#[derive(Debug, Clone)]
pub struct SubjectMap {}

#[derive(Debug, Clone)]
pub struct PredicateObjectMap {}

#[derive(Debug, Clone)]
pub struct GraphMap {}
