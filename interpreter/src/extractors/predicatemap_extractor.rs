use sophia_inmem::graph::FastGraph;

use super::{FromVocab, TermMapExtractor};
use crate::extractors::{extract_term_map_type_value, Extractor};
use crate::rml_model::term_map::{PredicateMap, TermMapInfo};
use crate::TermShared;

impl TermMapExtractor<PredicateMap> for PredicateMap {
    fn create_term_map(
        subj_ref: &TermShared,
        graph_ref: &FastGraph,
    ) -> super::ExtractorResult<PredicateMap> {
        let tm_info = TermMapInfo::extract(subj_ref, graph_ref)?;

        todo!()
    }

    fn create_constant_map(tm_info: TermMapInfo) -> PredicateMap {
        PredicateMap { tm_info }
    }

    fn get_map_pred() -> crate::TermString {
        vocab::r2rml::PROPERTY::PREDICATEMAP.to_term()
    }

    fn get_const_pred() -> crate::TermString {
        vocab::r2rml::PROPERTY::PREDICATE.to_term()
    }
}
