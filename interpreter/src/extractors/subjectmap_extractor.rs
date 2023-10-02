use sophia_api::term::TermKind;
use sophia_term::RcTerm;

use super::error::ParseError;
use super::store::get_objects;
use super::{Extractor, TermMapExtractor};
use crate::extractors::FromVocab;
use crate::rml_model::source_target::LogicalTarget;
use crate::rml_model::term_map::{SubjectMap, TermMapInfo};
use crate::IriString;

impl TermMapExtractor<SubjectMap> for SubjectMap {
    fn create_term_map(
        subj_ref: &RcTerm,
        graph_ref: &sophia_inmem::graph::FastGraph,
    ) -> super::ExtractorResult<SubjectMap> {
        let mut tm_info = TermMapInfo::extract_self(subj_ref, graph_ref)?;

        if tm_info.logical_targets.is_empty() {
            tm_info.logical_targets =
                vec![LogicalTarget::default()].into_iter().collect();
        }

        tm_info = match tm_info.term_type {
            Some(ttype)
                if ttype != TermKind::Iri && ttype != TermKind::BlankNode =>
            {
                return Err(ParseError::GenericError(
                    "PredicateMap can only have rr:Iri as rr:termType!"
                        .to_string(),
                ))
            }
            Some(_) => tm_info,
            None => {
                TermMapInfo {
                    term_type: Some(TermKind::Iri),
                    ..tm_info
                }
            }
        };

        let class_pred = vocab::r2rml::PROPERTY::CLASS.to_rcterm();

        let classes: Vec<IriString> =
            get_objects(graph_ref, subj_ref, &class_pred)
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

    fn get_map_pred() -> RcTerm {
        vocab::r2rml::PROPERTY::SUBJECTMAP.to_rcterm()
    }

    fn get_const_pred() -> RcTerm {
        vocab::r2rml::PROPERTY::SUBJECT.to_rcterm()
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::BufReader;
    use std::path::PathBuf;

    use sophia_api::graph::Graph;
    use sophia_api::triple::Triple;

    use crate::extractors::io::load_graph_bread;
    use crate::extractors::{ExtractorResult, FromVocab, TermMapExtractor};
    use crate::rml_model::term_map::{SubjectMap, TermMapType};
    use crate::{load_graph, test_case};

    #[test]
    fn create_subjectmap_test() -> ExtractorResult<()> {
        let graph = load_graph!("sample_mapping.ttl")?;
        let sub_pred = vocab::r2rml::PROPERTY::SUBJECTMAP.to_rcterm();
        let triple = graph.triples_with_p(&sub_pred).next().unwrap().unwrap();
        let sub_ref = triple.o();
        let subj_map = SubjectMap::create_term_map(sub_ref, &graph)?;

        assert_eq!(subj_map.tm_info.term_map_type, TermMapType::Template);
        assert!(subj_map.classes.len() == 1);

        Ok(())
    }
}
