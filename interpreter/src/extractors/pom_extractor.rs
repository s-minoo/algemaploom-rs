use super::Extractor;
use crate::extractors::FromVocab;
use crate::rml_model::term_map::PredicateObjectMap;

impl Extractor<PredicateObjectMap> for PredicateObjectMap {
    fn extract(
        subject_ref: &crate::TermShared,
        graph_ref: &sophia_inmem::graph::FastGraph,
    ) -> super::ExtractorResult<PredicateObjectMap> {
        let pm_pred1 = vocab::r2rml::PROPERTY::PREDICATE.to_term();
        let pm_pred2 = vocab::r2rml::PROPERTY::PREDICATEMAP.to_term();
        
        

        todo!()
    }
}
