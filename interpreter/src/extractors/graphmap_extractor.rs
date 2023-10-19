use sophia_api::term::TermKind;

use super::{Extractor, FromVocab, TermMapExtractor};
use crate::extractors::error::ParseError;
use crate::rml_model::term_map::{GraphMap, TermMapInfo};

impl TermMapExtractor<GraphMap> for GraphMap {
    fn get_term_map_info(&self) -> TermMapInfo {
        self.tm_info.clone()
    }

    fn create_constant_map(tm_info: TermMapInfo) -> GraphMap {
        if tm_info.term_type == Some(TermKind::Literal) {
            panic!("Constant-valued GraphMap has to be either an IRI or a BlankNode");
        }

        Self { tm_info }
    }

    fn create_term_map(
        subj_ref: &sophia_term::RcTerm,
        graph_ref: &sophia_inmem::graph::FastGraph,
    ) -> super::ExtractorResult<GraphMap> {
        let mut tm_info = TermMapInfo::extract_self(subj_ref, graph_ref)?;

        tm_info = match tm_info.term_type{
            Some(ttype) if ttype == TermKind::Literal || ttype== TermKind::Variable => {

                return Err(ParseError::GenericError(format!("GraphMap can only have either rr:Iri or rr:BNode as rr:termType!")))
            },
            Some(_) => tm_info,
            None => TermMapInfo{term_type: Some(TermKind::Iri), ..tm_info},
        };

        Ok(GraphMap { tm_info })
    }

    fn get_const_pred() -> sophia_term::RcTerm {
        vocab::r2rml::PROPERTY::GRAPH.to_rcterm()
    }

    fn get_map_pred() -> sophia_term::RcTerm {
        vocab::r2rml::PROPERTY::GRAPHMAP.to_rcterm()
    }
}
