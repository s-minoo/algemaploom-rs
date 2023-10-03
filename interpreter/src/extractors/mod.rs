use std::collections::HashSet;
use std::rc::Rc;

use lazy_static::lazy_static;
use regex::Regex;
use sophia_api::term::TTerm;
use sophia_inmem::graph::FastGraph;
use sophia_term::{RcTerm, Term};
use vocab::{ToString, PAIR};

use self::error::ParseError;
use crate::extractors::store::get_objects;
use crate::rml_model::term_map::{TermMapInfo, TermMapType};
use crate::TermString;

pub mod error;
mod functionmap_extractor;
pub mod io;
mod logicalsource_extractor;
mod logicaltarget_extractor;
mod objectmap_extractor;
mod pom_extractor;
mod predicatemap_extractor;
mod source;
mod store;
mod subjectmap_extractor;
mod term_map_info_extractor;
pub mod triplesmap_extractor;
mod util;

pub type ExtractorResult<T> = Result<T, ParseError>;

lazy_static! {
    static ref TEMPLATE_REGEX: Regex = Regex::new(r"\{([^\{\}]+)\}").unwrap();
}
fn prefix_attributes_from_template(template: &str, prefix: &str) -> String {
    let sanitized = template.replace("\\{", "\\(").replace("\\}", "\\)");
    TEMPLATE_REGEX
        .replace_all(&sanitized, format!("{{{}_$1}}", prefix))
        .replace("\\(", "\\{")
        .replace("\\)", "\\}")
}

fn get_attributes_from_template(template: &str) -> Vec<String> {
    let sanitized = template.replace("\\{", "").replace("\\}", "");
    let captured = TEMPLATE_REGEX.captures_iter(&sanitized);
    captured
        .filter_map(|cap| cap.get(1).map(|c| c.as_str().to_owned()))
        .collect()
}

pub trait TermMapExtractor<T> {
    fn prefix_attributes(&self, prefix: &str) -> TermMapInfo {
        let tm_info = self.get_term_map_info();
        let term_value = match tm_info.term_map_type {
            TermMapType::Constant => tm_info.term_value,
            TermMapType::Reference => {
                tm_info.term_value.map(|val| format!("{}_{}", prefix, val))
            }
            TermMapType::Template => {
                tm_info
                    .term_value
                    .map(|val| prefix_attributes_from_template(&val, prefix))
            }
            TermMapType::Function => todo!(),
        };

        TermMapInfo {
            term_value,
            ..tm_info
        }
    }

    fn get_attributes(&self) -> HashSet<String> {
        let tm_info = self.get_term_map_info();
        let value = tm_info.term_value.value().to_string();
        match tm_info.term_map_type {
            TermMapType::Constant => HashSet::new(),
            TermMapType::Reference => vec![value].into_iter().collect(),
            TermMapType::Template => {
                get_attributes_from_template(&value).into_iter().collect()
            }
            TermMapType::Function => todo!(),
        }
    }
    fn get_term_map_info(&self) -> TermMapInfo;

    fn create_constant_map(tm_info: TermMapInfo) -> T;

    fn create_term_map(
        subj_ref: &RcTerm,
        graph_ref: &FastGraph,
    ) -> ExtractorResult<T>;

    fn extract_constant_term_map(map_const: &Term<Rc<str>>) -> T {
        let tm_info = TermMapInfo::from_constant_value(map_const.clone());

        Self::create_constant_map(tm_info)
    }

    fn extract_from_container(
        graph_ref: &FastGraph,
        container_map_subj_ref: &RcTerm,
    ) -> ExtractorResult<T> {
        Self::extract_many_from_container(graph_ref, container_map_subj_ref)
            .and_then(|mut vec| vec.pop().ok_or(ParseError::Infallible))
    }

    fn extract_many_from_container(
        graph_ref: &FastGraph,
        container_map_subj_ref: &RcTerm,
    ) -> ExtractorResult<Vec<T>> {
        let map_pred = Self::get_map_pred();
        let const_pred = Self::get_const_pred();
        let map_subj_vec =
            get_objects(graph_ref, container_map_subj_ref, &map_pred);
        let map_const_obj_vec =
            get_objects(graph_ref, container_map_subj_ref, &const_pred);

        if !map_subj_vec.is_empty() {
            return Ok(map_subj_vec
                .iter()
                .flat_map(|map_subj| Self::create_term_map(map_subj, graph_ref))
                .collect());
        } else if !map_const_obj_vec.is_empty() {
            return Ok(map_const_obj_vec
                .iter()
                .map(|map_const_obj_vec| {
                    Self::extract_constant_term_map(map_const_obj_vec)
                })
                .collect::<Vec<_>>());
        }

        Err(ParseError::GenericError(format!(
            "TriplesMap {} has no subject map!",
            container_map_subj_ref
        )))
    }

    fn get_const_pred() -> RcTerm;
    fn get_map_pred() -> RcTerm;
}

pub trait Extractor<T> {
    fn extract_identifier(subj_ref: &RcTerm) -> Result<TermString, ParseError> {
        let identifier =
            subj_ref.to_owned().map(|i| i.to_string()).try_into()?;
        Ok(identifier)
    }

    fn extract_self(
        subject_ref: &RcTerm,
        graph_ref: &FastGraph,
    ) -> ExtractorResult<T>;
}

pub trait FromVocab {
    fn to_rcterm(&self) -> RcTerm;

    fn to_term(&self) -> Term<String>;
}

impl<'a> FromVocab for PAIR<'a> {
    fn to_term(&self) -> Term<String> {
        Term::new_iri(self.to_string()).unwrap()
    }
    fn to_rcterm(&self) -> RcTerm {
        Term::new_iri(self.to_string().as_ref()).unwrap()
    }
}
