use sophia_api::term::TermKind;
use sophia_inmem::graph::FastGraph;

use super::{FromVocab, TermMapExtractor};
use crate::extractors::store::get_object;
use crate::extractors::Extractor;
use crate::rml_model::term_map::{ObjectMap, TermMapInfo};
use crate::{IriString, TermShared};


impl TermMapExtractor<ObjectMap> for ObjectMap {
    fn create_constant_map(tm_info: TermMapInfo) -> ObjectMap {
        if tm_info.term_type == Some(TermKind::BlankNode) {
            panic!("Constant-valued ObjectMap has to have an IRI or a Literal as value");
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
        subj_ref: &TermShared,
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

        let tm_info = TermMapInfo::extract(subj_ref, graph_ref)?;

        Ok(ObjectMap {
            tm_info,
            parent_tm: None,
            join_condition: None,
            data_type,
            language,
        })
    }

    fn get_const_pred() -> crate::TermShared {
        vocab::r2rml::PROPERTY::OBJECT.to_term()
    }

    fn get_map_pred() -> crate::TermShared {
        vocab::r2rml::PROPERTY::OBJECTMAP.to_term()
    }
}
