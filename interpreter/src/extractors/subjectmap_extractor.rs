use std::collections::HashSet;

use sophia_api::graph::Graph;
use sophia_api::triple::Triple;
use sophia_term::iri::Iri;

use super::Extractor;
use crate::extractors::FromVocab;
use crate::rml_model::term_map::{SubjectMap, TermMapInfo, TermMapType};

impl Extractor<SubjectMap> for SubjectMap {
    fn extract(
        subject: &super::TermShared,
        graph: &sophia_inmem::graph::FastGraph,
    ) -> super::ExtractorResult<SubjectMap> {
        let (term_map_type, term_value) =
            Self::extract_term_map_type_value(subject, graph)?;

        let identifier =
            subject.to_owned().map(|i| i.to_string()).try_into()?;

        let tm_info = TermMapInfo {
            identifier,
            logical_targets: HashSet::new(),
            term_map_type,
            term_value,
        };

        let class_pred = vocab::r2rml::PROPERTY::CLASS.to_term();

        let classes: Vec<_> = graph
            .triples_with_sp(subject, &class_pred)
            .filter_map(|trip_res| {
                trip_res.map(|trip| trip.o().to_owned()).ok()
            })
            .filter_map(|term| {
                term.map(|inner| inner.to_string()).try_into().ok()
            })
            .collect();

        Ok(SubjectMap { tm_info, classes })
    }
}
