pub const PREFIX: &str = "rml";

pub const IRI: &str = "http://w3id.org/rml/";

pub mod PROPERTY {
    use super::IRI;
    use crate::PAIR;

    pub const SUBJECTMAP: PAIR = (IRI, "subjectMap");
}