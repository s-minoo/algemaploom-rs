use std::error::Error;
use std::rc::Rc;

use sophia_api::graph::Graph;
use sophia_api::triple::streaming_mode::{ByTermRefs, StreamedTriple};
use sophia_api::triple::Triple;
use sophia_inmem::graph::FastGraph;
use sophia_term::matcher::ANY;
use sophia_term::Term;
use vocab::{ToString, PAIR};

use self::error::ParseError;
use crate::rml_model::term_map::TermMapType;
use crate::{TermShared, TermString};

pub mod error;
pub mod logicalsource_extractor;
pub mod pom_extractor;
pub mod store;
pub mod subjectmap_extractor;
pub mod triplesmap_extractor;

type ExtractorResult<T> = Result<T, ParseError>;
type Triples = StreamedTriple<'static, ByTermRefs<Term<Rc<str>>>>;

pub fn extract_term_map_type_value(
    subject_ref: &TermShared,
    graph_ref: &FastGraph,
) -> ExtractorResult<(TermMapType, TermString)> {
    //template-map
    let temp_pred: TermString = vocab::r2rml::PROPERTY::TEMPLATE.to_term();

    //constant-map
    let const_pred: TermString = vocab::r2rml::PROPERTY::CONSTANT.to_term();

    //reference-map
    let ref_pred: TermString = vocab::rml::PROPERTY::REFERENCE.to_term();
    let col_pred: TermString = vocab::r2rml::PROPERTY::COLUMN.to_term();

    let pred_query = &[&ref_pred, &col_pred, &const_pred, &temp_pred];

    let mut results_query: Vec<_> = graph_ref
        .triples_matching(subject_ref, pred_query, &ANY)
        .filter_map(|trip| trip.ok())
        .collect();

    if results_query.len() > 1 {
        return Err(ParseError::GenericError(
                    format!("More than one occurences of rr:template, rml:reference, rr:constant, or rr:column")
                    ));
    }

    let trip = results_query.pop().unwrap();
    let fetched_pred = trip.p();

    let term_map_type_res = match fetched_pred {
        ref_map if *ref_map == ref_pred || *ref_map == col_pred => {
            Ok(TermMapType::Reference)
        }
        const_map if *const_map == const_pred => Ok(TermMapType::Constant),
        temp_map if *temp_map == temp_pred => Ok(TermMapType::Template),
        _ => Err(ParseError::Infallible),
    };

    let term_value = trip.o().to_owned().map(|i| i.to_string());

    term_map_type_res.map(|map_type| (map_type, term_value))
}

pub trait Extractor<T> {
    fn extract(
        subject_ref: &TermShared,
        graph_ref: &FastGraph,
    ) -> ExtractorResult<T>;
}

pub trait FromVocab {
    fn to_term(&self) -> TermString;
}

impl<'a> FromVocab for PAIR<'a> {
    fn to_term(&self) -> TermString {
        Term::new_iri(self.to_string()).unwrap()
    }
}
