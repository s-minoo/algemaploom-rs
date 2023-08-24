use sophia_api::term::TTerm;
use sophia_inmem::graph::FastGraph;

use super::error::ParseError;
use super::{Extractor, ExtractorResult, RcTerm};
use crate::extractors::store::get_object;
use crate::extractors::FromVocab;
use crate::rml_model::source_target::{LogicalSource, Source};

impl Extractor<LogicalSource> for LogicalSource {
    fn extract_self(
        subject: &RcTerm,
        graph: &FastGraph,
    ) -> super::ExtractorResult<LogicalSource> {
        let iter_pred = vocab::rml::PROPERTY::ITERATOR.to_term();
        let refform_pred = vocab::rml::PROPERTY::REFERENCEFORMULATION.to_term();

        let iterator = get_object(graph, subject, &iter_pred)
            .ok()
            .map(|it| it.to_string());

        let reference_formulation = get_object(graph, subject, &refform_pred)?
            .map(|inner| (*inner).to_string())
            .try_into()?;

        let input = extract_input_type(subject, graph)?;

        Ok(LogicalSource {
            identifier: subject.to_owned().map(|i|i.to_string()),
            iterator,
            source: input,
            reference_formulation,
        })
    }
}

// TODO: expand to also support other input types <05-04-23,> //
fn extract_input_type(
    subject: &RcTerm,
    graph: &FastGraph,
) -> ExtractorResult<Source> {
    let source_pred = vocab::rml::PROPERTY::SOURCE.to_term();
    let source = get_object(graph, subject, &source_pred)?;

    match source.kind() {
        sophia_api::term::TermKind::Literal => {
            Ok(Source::FileInput {
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

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::BufReader;
    use std::path::PathBuf;

    use sophia_api::graph::Graph;
    use sophia_api::triple::Triple;

    use super::*;
    use crate::extractors::io::load_graph_bread;
    use crate::extractors::ExtractorResult;
    use crate::{load_graph, test_case};

    #[test]
    fn logical_source_extract_test() -> ExtractorResult<()> {
        let graph: FastGraph = load_graph!("sample_mapping.ttl")?;
        let sub_pred = vocab::rml::PROPERTY::LOGICALSOURCE.to_term();
        let triple = graph.triples_with_p(&sub_pred).next().unwrap().unwrap();

        let sub_ref = triple.o();
        let logical_source = LogicalSource::extract_self(sub_ref, &graph)?;

        assert_eq!(
            logical_source.reference_formulation,
            vocab::query::CLASS::CSV.to_term()
        );
        assert!(logical_source.iterator.is_none());
        Ok(())
    }

    #[test]
    fn input_type_test() -> ExtractorResult<()> {
        let graph: FastGraph = load_graph!("sample_mapping.ttl")?;
        let sub_pred = vocab::rml::PROPERTY::LOGICALSOURCE.to_term();
        let triple = graph.triples_with_p(&sub_pred).next().unwrap().unwrap();

        let sub_ref = triple.o();
        let input_type = extract_input_type(sub_ref, &graph)?;

        assert!(
            input_type
                == Source::FileInput {
                    path: "Airport.csv".to_string(),
                }
        );

        Ok(())
    }
}
