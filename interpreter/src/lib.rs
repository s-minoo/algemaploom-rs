use std::rc::Rc;

use sophia_term::blank_node::BlankNode;
use sophia_term::iri::Iri;
use sophia_term::literal::Literal;
use sophia_term::Term;

pub mod extractors;
pub mod rml_model;

type TermShared = Term<Rc<str>>;
type TermString = Term<String>;
type IriString = Iri<String>;
type LiteralString = Literal<String>;
type BNodeString = BlankNode<String>;

