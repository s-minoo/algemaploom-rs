use sophia_api::graph::Graph;
use sophia_api::triple::Triple;
use sophia_inmem::graph::FastGraph;
use sophia_term::Term;
use vocab::ToString;

use super::error::ParseError;
use super::{Extractor, ExtractorResult, RcTerm};
use crate::extractors::store::{get_object, get_objects};
use crate::extractors::{FromVocab, TermMapExtractor};
use crate::rml_model::source_target::LogicalSource;
use crate::rml_model::term_map::{PredicateObjectMap, SubjectMap, TriplesMap};

impl Extractor<TriplesMap> for TriplesMap {
    fn extract(
        subject: &RcTerm,
        graph: &FastGraph,
    ) -> ExtractorResult<TriplesMap> {
        let subject_map = SubjectMap::extract_term_map(graph, subject)?;

        let ls_term = vocab::rml::PROPERTY::LOGICALSOURCE.to_term();
        let logical_source_subj = get_object(graph, subject, &ls_term)?;
        let logical_source =
            LogicalSource::extract(&logical_source_subj, graph)?;

        let pom = vocab::r2rml::PROPERTY::PREDICATEOBJECTMAP.to_term();
        let po_maps: Vec<_> = get_objects(graph, subject, &pom)
            .into_iter()
            .filter_map(|pom_subj| {
                PredicateObjectMap::extract(&pom_subj, graph).ok()
            })
            .collect();

        Ok(TriplesMap {
            identifier: subject.to_owned().map(|i| i.to_string()),
            logical_source,
            subject_map,
            po_maps,
            graph_map: None,
        })
    }
}

pub fn extract_triples_maps(
    graph: &FastGraph,
) -> Result<Vec<TriplesMap>, ParseError> {
    let ptype: RcTerm = Term::new_iri(vocab::rdf::PROPERTY::TYPE.to_string())?;
    let otm: RcTerm =
        Term::new_iri(vocab::r2rml::CLASS::TRIPLESMAP.to_string())?;

    Ok(graph
        .triples_with_po(&ptype, &otm)
        .filter_map(|triple| triple.ok())
        .filter_map(|triple| TriplesMap::extract(triple.s(), graph).ok())
        .collect())
}
