use std::rc::Rc;

use sophia_term::{Term, iri::Iri, literal::Literal, blank_node::BlankNode};

pub mod extractors;
pub mod rml_model;

type TermShared = Term<Rc<str>>;
type TermString = Term<String>;
type IriString = Iri<String>;
type LiteralString = Literal<String>;
type BNodeString = BlankNode<String>;
