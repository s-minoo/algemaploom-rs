pub const PREFIX: &str = "formats";
pub const IRI: &str = "http://www.w3.org/ns/formats/";

pub mod CLASS {
    use super::IRI;
    use crate::PAIR;
    pub const NTRIPLES: PAIR = (IRI, "N-Triples");
    pub const NQUADS: PAIR = (IRI, "N-Quads");
    pub const JSONLD: PAIR = (IRI, "JSON-LD");
}
