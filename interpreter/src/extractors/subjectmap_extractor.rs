use sophia_api::term::TermKind;

use super::store::get_objects;
use super::{Extractor, TermMapExtractor};
use crate::extractors::FromVocab;
use crate::rml_model::term_map::{SubjectMap, TermMapInfo};
use crate::{IriString, TermShared};

impl TermMapExtractor<SubjectMap> for SubjectMap {
    fn create_term_map(
        subj_ref: &crate::TermShared,
        graph_ref: &sophia_inmem::graph::FastGraph,
    ) -> super::ExtractorResult<SubjectMap> {
        let tm_info = TermMapInfo::extract(subj_ref, graph_ref)?;

        let class_pred = vocab::r2rml::PROPERTY::CLASS.to_term();

        let classes: Vec<IriString> =
            get_objects(graph_ref, subj_ref, &class_pred)?
                .iter()
                .map(|item| item.try_into().unwrap())
                .collect();

        Ok(SubjectMap { tm_info, classes })
    }

    fn create_constant_map(tm_info: TermMapInfo) -> SubjectMap {
        if tm_info.term_type != Some(TermKind::Iri) {
            panic!("Constant-valued SubjectMap has to have an IRI as value");
        }
        SubjectMap {
            tm_info,
            classes: Vec::new(),
        }
    }

    fn get_map_pred() -> TermShared {
        vocab::r2rml::PROPERTY::SUBJECTMAP.to_term()
    }

    fn get_const_pred() -> TermShared {
        vocab::r2rml::PROPERTY::SUBJECT.to_term()
    }
}
