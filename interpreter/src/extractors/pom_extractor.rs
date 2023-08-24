use sophia_term::RcTerm;

use super::Extractor;
use crate::extractors::TermMapExtractor;
use crate::rml_model::term_map::{ObjectMap, PredicateMap};
use crate::rml_model::PredicateObjectMap;

impl Extractor<PredicateObjectMap> for PredicateObjectMap {
    fn extract_self(
        subject_ref: &RcTerm,
        graph_ref: &sophia_inmem::graph::FastGraph,
    ) -> super::ExtractorResult<PredicateObjectMap> {
        let predicate_maps =
            PredicateMap::extract_many_from_container(graph_ref, subject_ref)?;

        let object_maps = ObjectMap::extract_many_from_container(graph_ref, subject_ref)?;

        Ok(PredicateObjectMap {
            predicate_maps,
            object_maps,
        })
    }
}
