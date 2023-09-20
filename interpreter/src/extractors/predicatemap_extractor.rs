use sophia_api::term::TermKind;
use sophia_inmem::graph::FastGraph;
use sophia_term::RcTerm;

use super::{FromVocab, TermMapExtractor};
use crate::extractors::error::ParseError;
use crate::extractors::Extractor;
use crate::rml_model::term_map::{PredicateMap, TermMapInfo};

impl TermMapExtractor<PredicateMap> for PredicateMap {
    fn create_term_map(
        subj_ref: &RcTerm,
        graph_ref: &FastGraph,
    ) -> super::ExtractorResult<PredicateMap> {
        let mut tm_info = TermMapInfo::extract_self(subj_ref, graph_ref)?;

        tm_info = match tm_info.term_type {
            Some(ttype) if ttype != TermKind::Iri => {
                return Err(ParseError::GenericError("PredicateMap can only have rr:Iri as rr:termType!".to_string()))
            }
            Some(_) => tm_info,
            None => {
                TermMapInfo {
                    term_type: Some(TermKind::Iri),
                    ..tm_info
                }
            }
        };

        Ok(PredicateMap { tm_info })
    }

    fn create_constant_map(tm_info: TermMapInfo) -> PredicateMap {
        if tm_info.term_type != Some(TermKind::Iri) {
            panic!("Constant-valued PredicateMap has to have an IRI as value");
        }
        PredicateMap { tm_info }
    }

    fn get_map_pred() -> RcTerm {
        vocab::r2rml::PROPERTY::PREDICATEMAP.to_term()
    }

    fn get_const_pred() -> RcTerm {
        vocab::r2rml::PROPERTY::PREDICATE.to_term()
    }
}

#[cfg(test)]
mod tests {
    

    use super::*;
    use crate::import_test_mods;
    use crate::rml_model::term_map::TermMapType;

    import_test_mods!();

    #[test]
    fn create_const_predicatemap_test() -> ExtractorResult<()> {
        let graph = load_graph!("sample_mapping.ttl")?;
        let pm_const_pred = vocab::r2rml::PROPERTY::PREDICATE.to_term();
        let triples = graph.triples_with_p(&pm_const_pred);
        let values = triples.flatten().map(|trip| trip.o().to_owned());
        let pms: Vec<PredicateMap> = values
            .map(|map_const| {
                PredicateMap::extract_constant_term_map(&map_const)
            })
            .collect();

        assert_eq!(pms.len(), 3);

        pms.iter().for_each(|pm| {
            assert_eq!(pm.tm_info.term_map_type, TermMapType::Constant);
            assert_eq!(pm.tm_info.term_type, Some(TermKind::Iri));
        });

        Ok(())
    }
}
