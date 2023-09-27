pub const PREFIX: &str = "rdf";
pub const IRI: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";

pub mod PROPERTY {
    use super::IRI;
    use crate::PAIR;

    pub const TYPE: PAIR = (IRI, "type");
}

pub mod CLASS {
    use super::IRI;
    use crate::PAIR;

    pub const RDF_OBJECT: PAIR = (IRI, "Object");
    pub const RDF_LIST: PAIR = (IRI, "List");
}
