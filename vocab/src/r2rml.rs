pub const PREFIX: &str = "rml";
pub const IRI: &str = "http://semweb.mmlab.be/ns/rml#";

pub mod PROPERTY {
    use super::IRI;
    use crate::PAIR;

    pub const PREDICATEOBJECTMAP: PAIR = (IRI, "predicateObjectMap");
    pub const PREDICATE: PAIR = (IRI, "predicate");
    pub const PREDICATEMAP: PAIR = (IRI, "predicateMap");
    pub const OBJECT: PAIR = (IRI, "object");
    pub const OBJECTMAP: PAIR = (IRI, "objectMap");
    pub const TRIPLESMAP: PAIR = (IRI, "triplesMap");
    pub const SUBJECTMAP: PAIR = (IRI, "subjectMap");
    pub const SUBJECT: PAIR = (IRI, "subject");
    pub const CONSTANT: PAIR = (IRI, "constant");
    pub const TEMPLATE: PAIR = (IRI, "template");
    pub const TERMTYPE: PAIR = (IRI, "termType");
    pub const CLASS: PAIR = (IRI, "class");
    pub const PARENTTRIPLESMAP: PAIR = (IRI, "parentTriplesMap");
    pub const JOINCONDITION: PAIR = (IRI, "joinCondition");
    pub const PARENT: PAIR = (IRI, "parent");
    pub const CHILD: PAIR = (IRI, "child");
    pub const GRAPH: PAIR = (IRI, "graph");
    pub const GRAPHMAP: PAIR = (IRI, "graphMap");
    pub const DATATYPE: PAIR = (IRI, "datatype");
    pub const LANGUAGE: PAIR = (IRI, "language");
    pub const DEFAULTGRAPH: PAIR = (IRI, "defaultGraph");
}

pub mod CLASS {
    use super::IRI;
    use crate::PAIR;
    pub const PREDICATEOBJECTMAP: PAIR = (IRI, "PredicateObjectMap");
    pub const OBJECTMAP: PAIR = (IRI, "ObjectMap");
    pub const TRIPLESMAP: PAIR = (IRI, "TriplesMap");
    pub const IRI: PAIR = (IRI, "IRI");
    pub const BLANKNODE: PAIR = (IRI, "BlankNode");
    pub const LITERAL: PAIR = (IRI, "Literal");
}
