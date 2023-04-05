use std::collections::HashSet;

use sophia_api::graph::Graph;
use sophia_api::term::{TTerm, TermKind};
use sophia_inmem::graph::FastGraph;

use super::error::ParseError;
use super::store::get_object;
use super::{extract_term_map_type_value, Extractor, FromVocab};
use crate::rml_model::term_map::TermMapInfo;

impl Extractor<TermMapInfo> for TermMapInfo {
    fn extract(
        subj_ref: &crate::TermShared,
        graph_ref: &FastGraph,
    ) -> super::ExtractorResult<TermMapInfo> {
        let (term_map_type, term_value) =
            extract_term_map_type_value(subj_ref, graph_ref)?;

        let term_type_pred = vocab::r2rml::PROPERTY::TERMTYPE.to_term();

        let term_type_soph = get_object(graph_ref, subj_ref, &term_type_pred)?;

        let lit_class = vocab::r2rml::CLASS::LITERAL.to_term();
        let iri_class = vocab::r2rml::CLASS::IRI.to_term();
        let bnode_class = vocab::r2rml::CLASS::BLANKNODE.to_term();

        let term_type = match term_type_soph {
            sophia_term::Term::Iri(iri) if iri == iri_class => {
                Some(TermKind::Iri)
            }
            sophia_term::Term::Iri(iri) if iri == bnode_class => {
                Some(TermKind::BlankNode)
            }
            sophia_term::Term::Iri(iri) if iri == lit_class => {
                Some(TermKind::Literal)
            }
            _ => None,
        };

        let identifier =
            subj_ref.to_owned().map(|i| i.to_string()).try_into()?;

        Ok(TermMapInfo {
            identifier,
            logical_targets: HashSet::new(),
            term_map_type,
            term_value,
            term_type,
        })
    }
}
