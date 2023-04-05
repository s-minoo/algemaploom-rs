use std::collections::HashSet;

use sophia_api::graph::Graph;
use sophia_api::triple::Triple;
use sophia_term::iri::Iri;

use super::store::get_objects;
use super::{extract_term_map_type_value, Extractor, TermMapExtractor};
use crate::extractors::FromVocab;
use crate::rml_model::term_map::{SubjectMap, TermMapInfo, TermMapType};
use crate::{IriString, TermString};

impl TermMapExtractor<SubjectMap> for SubjectMap {
    fn create_term_map(
        subj_ref: &crate::TermShared,
        graph_ref: &sophia_inmem::graph::FastGraph,
    ) -> super::ExtractorResult<SubjectMap> {
        SubjectMap::extract(subj_ref, graph_ref)
    }

    fn create_constant_map(tm_info: TermMapInfo) -> SubjectMap {
        SubjectMap {
            tm_info,
            classes: Vec::new(),
        }
    }

    fn get_map_pred() -> TermString {
        vocab::r2rml::PROPERTY::SUBJECTMAP.to_term()
    }

    fn get_const_pred() -> TermString {
        vocab::r2rml::PROPERTY::SUBJECT.to_term()
    }
}

impl Extractor<SubjectMap> for SubjectMap {
    fn extract(
        subject: &super::TermShared,
        graph: &sophia_inmem::graph::FastGraph,
    ) -> super::ExtractorResult<SubjectMap> {
        let (term_map_type, term_value) =
            extract_term_map_type_value(subject, graph)?;

        let identifier =
            subject.to_owned().map(|i| i.to_string()).try_into()?;

        let tm_info = TermMapInfo {
            identifier,
            logical_targets: HashSet::new(),
            term_map_type,
            term_value,
        };

        let class_pred = vocab::r2rml::PROPERTY::CLASS.to_term();

        let classes: Vec<IriString> = get_objects(graph, subject, &class_pred)?
            .iter()
            .map(|item| item.try_into().unwrap())
            .collect();

        Ok(SubjectMap { tm_info, classes })
    }
}
