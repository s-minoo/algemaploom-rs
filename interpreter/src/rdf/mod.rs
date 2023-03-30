use interpreter::rml_model::term::{Resource, Term, IRI};

#[derive(Debug, Clone)]
pub struct Triple {
    pub subject:   Resource,
    pub predicate: IRI,
    pub object:    Term,
}

#[derive(Debug, Clone)]
pub struct Quad {
    pub triple: Triple,
    pub graph:  Option<IRI>,
}
