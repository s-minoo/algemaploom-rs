use sophia_api::term::TTerm;
use sophia_inmem::graph::FastGraph;

use super::{Extractor, ExtractorResult, RcTerm};
use crate::extractors::store::get_object;
use crate::extractors::FromVocab;
use crate::rml_model::source_target::{LogicalSource, Source};

impl Extractor<LogicalSource> for LogicalSource {
    fn extract_self(
        subject: &RcTerm,
        graph: &FastGraph,
    ) -> super::ExtractorResult<LogicalSource> {
        let iter_pred = vocab::rml::PROPERTY::ITERATOR.to_rcterm();
        let refform_pred =
            vocab::rml::PROPERTY::REFERENCEFORMULATION.to_rcterm();

        let iterator = get_object(graph, subject, &iter_pred)
            .ok()
            .map(|it| it.value().to_string());

        let reference_formulation = get_object(graph, subject, &refform_pred)?
            .map(|inner| (*inner).to_string())
            .try_into()?;

        let source = extract_concrete_source(subject, graph)?;

        Ok(LogicalSource {
            identifier: subject.to_string(),
            iterator,
            source,
            reference_formulation,
        })
    }
}

fn extract_concrete_source(
    subject: &RcTerm,
    graph: &FastGraph,
) -> ExtractorResult<Source> {
    let source_pred = vocab::rml::PROPERTY::SOURCE.to_rcterm();
    let source_subj = get_object(graph, subject, &source_pred)?;

    Source::extract_self(&source_subj, graph)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::PathBuf;

    use sophia_api::graph::Graph;
    use sophia_api::triple::Triple;

    use super::*;
    use crate::extractors::io::load_graph_bread;
    use crate::extractors::ExtractorResult;
    use crate::rml_model::source_target::SourceType;
    use crate::{load_graph, test_case};

    #[test]
    fn logical_source_extract_test() -> ExtractorResult<()> {
        let graph: FastGraph = load_graph!("sample_mapping.ttl")?;
        let sub_pred = vocab::rml::PROPERTY::LOGICALSOURCE.to_rcterm();
        let triple = graph.triples_with_p(&sub_pred).next().unwrap().unwrap();

        let sub_ref = triple.o();
        let logical_source = LogicalSource::extract_self(sub_ref, &graph)?;

        assert_eq!(
            logical_source.reference_formulation,
            vocab::query::CLASS::CSV.to_rcterm()
        );
        assert!(logical_source.iterator.is_none());
        Ok(())
    }

    #[test]
    fn input_type_test() -> ExtractorResult<()> {
        let graph: FastGraph = load_graph!("sample_mapping.ttl")?;
        let sub_pred = vocab::rml::PROPERTY::LOGICALSOURCE.to_rcterm();
        let triple = graph.triples_with_p(&sub_pred).next().unwrap().unwrap();

        let sub_ref = triple.o();
        let generated = extract_concrete_source(sub_ref, &graph)?;

        let config = HashMap::from_iter(vec![(
            "path".to_string(),
            "shoes.csv".to_string(),
        )]);

        let expected = Source {
            source_type: SourceType::FileInput,
            config,
        };
        assert!(
            generated == expected,
            "Generated: {:?} \n Expected: {:?}",
            generated,
            expected
        );

        Ok(())
    }
}
