use sophia_api::term::{TTerm, TermKind};
use sophia_inmem::graph::FastGraph;
use sophia_term::RcTerm;

use super::{FromVocab, TermMapExtractor};
use crate::extractors::store::get_object;
use crate::extractors::Extractor;
use crate::rml_model::term_map::{ObjectMap, TermMapInfo};
use crate::IriString;

impl TermMapExtractor<ObjectMap> for ObjectMap {
    fn create_constant_map(mut tm_info: TermMapInfo) -> ObjectMap {
        if tm_info.term_type == Some(TermKind::BlankNode) {
            panic!("Constant-valued ObjectMap has to have an IRI or a Literal as value");
        }

        if tm_info.term_type.is_none() {
            tm_info.term_type = Some(tm_info.term_value.kind());
        }

        ObjectMap {
            tm_info,
            parent_tm: None,
            join_condition: None,
            data_type: None,
            language: None,
        }
    }
    fn create_term_map(
        subj_ref: &RcTerm,
        graph_ref: &FastGraph,
    ) -> super::ExtractorResult<ObjectMap> {
        // TODO:Implement term map join parsing correctly!  <05-04-23, Min Oo> //

        let dtype_pred = vocab::r2rml::PROPERTY::DATATYPE.to_term();
        let data_type: Option<IriString> =
            get_object(graph_ref, subj_ref, &dtype_pred)
                .ok()
                .map(|tshared| tshared.map(|i| i.to_string()))
                .and_then(|tstring| tstring.try_into().ok());

        let lang_pred = vocab::r2rml::PROPERTY::LANGUAGE.to_term();
        let language = get_object(graph_ref, subj_ref, &lang_pred)
            .ok()
            .map(|tshared| tshared.to_string());

        let mut tm_info = TermMapInfo::extract(subj_ref, graph_ref)?;

        if tm_info.term_type.is_none() {
            tm_info.term_type = Some(tm_info.term_value.kind());
        }

        Ok(ObjectMap {
            tm_info,
            parent_tm: None,
            join_condition: None,
            data_type,
            language,
        })
    }

    fn get_const_pred() -> RcTerm {
        vocab::r2rml::PROPERTY::OBJECT.to_term()
    }

    fn get_map_pred() -> RcTerm {
        vocab::r2rml::PROPERTY::OBJECTMAP.to_term()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::import_test_mods;
    use crate::rml_model::term_map::TermMapType;

    import_test_mods!();

    #[test]
    fn map_object_test() -> ExtractorResult<()> {
        let graph: FastGraph = load_graph!("sample_mapping.ttl")?;
        let map_pred = vocab::r2rml::PROPERTY::OBJECTMAP.to_term();
        let container_vec = graph
            .triples_with_p(&map_pred)
            .flatten()
            .map(|trip| trip.s().to_owned());

        let obj_maps: Vec<_> = container_vec
            .flat_map(|objmap_container| {
                ObjectMap::extract_term_maps(&graph, &objmap_container)
            })
            .flatten()
            .collect();

        assert_eq!(obj_maps.len(), 3);
        obj_maps.iter().for_each(|om| {
            assert_eq!(om.tm_info.term_type, Some(TermKind::Literal));
            assert_eq!(om.tm_info.term_map_type, TermMapType::Reference);
        });

        Ok(())
    }
}
