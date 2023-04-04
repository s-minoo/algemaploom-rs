use std::rc::Rc;

use sophia_api::term::TTerm;
use sophia_inmem::graph::FastGraph;
use sophia_term::iri::Iri;
use sophia_term::Term;

use super::error::ParseError;
use super::{Extractor, ExtractorResult, TermShared};
use crate::extractors::store::get_object;
use crate::extractors::FromVocab;
use crate::rml_model::source_target::{Input, LogicalSource};

impl Extractor<LogicalSource> for LogicalSource {
    fn extract(
        subject: &TermShared,
        graph: &FastGraph,
    ) -> super::ExtractorResult<LogicalSource> {
        let iter_pred = vocab::rml::PROPERTY::ITERATOR.to_term();
        let refform_pred = vocab::rml::PROPERTY::REFERENCEFORMULATION.to_term();

        let iterator = get_object(graph, subject, &iter_pred)?.to_string();
        let reference_formulation = get_object(graph, subject, &refform_pred)?
            .map(|inner| (*inner).to_string())
            .try_into()?;
        let input = extract_input_type(subject, graph)?;

        Ok(LogicalSource {
            identifier: subject.to_string(),
            iterator,
            input,
            reference_formulation,
        })
    }
}

fn extract_input_type(
    subject: &TermShared,
    graph: &FastGraph,
) -> ExtractorResult<Input> {
    let source_pred = vocab::rml::PROPERTY::SOURCE.to_term();
    let source = get_object(graph, subject, &source_pred)?;

    match source.kind() {
        sophia_api::term::TermKind::Literal => {
            Ok(Input::FileInput {
                path: source.value().to_string(),
            })
        }
        sophia_api::term::TermKind::Iri => todo!(),
        sophia_api::term::TermKind::BlankNode => todo!(),
        sophia_api::term::TermKind::Variable => {
            Err(ParseError::GenericError(
                "Source can't be a variable".to_string(),
            ))
        }
    }
}
