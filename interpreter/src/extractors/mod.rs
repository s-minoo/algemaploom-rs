use std::rc::Rc;

use sophia_api::graph::Graph;
use sophia_api::triple::Triple;
use sophia_inmem::graph::FastGraph;
use sophia_term::matcher::ANY;
use sophia_term::{Term, RcTerm};
use vocab::{ToString, PAIR};

use self::error::ParseError;
use crate::extractors::store::get_objects;
use crate::rml_model::term_map::{TermMapInfo, TermMapType};
use crate::TermString;

pub mod error;
pub mod io;
mod logicalsource_extractor;
mod objectmap_extractor;
mod pom_extractor;
mod predicatemap_extractor;
mod store;
mod subjectmap_extractor;
mod term_map_info_extractor;
pub mod triplesmap_extractor;

pub type ExtractorResult<T> = Result<T, ParseError>;

pub fn extract_term_map_type_value(
    subject_ref: &RcTerm,
    graph_ref: &FastGraph,
) -> ExtractorResult<(TermMapType, TermString)> {
    //template-map
    let temp_pred: RcTerm = vocab::r2rml::PROPERTY::TEMPLATE.to_term();

    //constant-map
    let const_pred: RcTerm = vocab::r2rml::PROPERTY::CONSTANT.to_term();

    //reference-map
    let ref_pred: RcTerm = vocab::rml::PROPERTY::REFERENCE.to_term();
    let col_pred: RcTerm = vocab::r2rml::PROPERTY::COLUMN.to_term();

    let pred_query = &[&ref_pred, &col_pred, &const_pred, &temp_pred];

    let mut results_query: Vec<_> = graph_ref
        .triples_matching(subject_ref, pred_query, &ANY)
        .filter_map(|trip| trip.ok())
        .collect();

    if results_query.len() > 1 {
        return Err(ParseError::GenericError(
                    format!("More than one occurences of rr:template, rml:reference, rr:constant, or rr:column")
                    ));
    }

    let trip = results_query.pop().unwrap();
    let fetched_pred = trip.p();

    let term_map_type_res = match fetched_pred {
        ref_map if *ref_map == ref_pred || *ref_map == col_pred => {
            Ok(TermMapType::Reference)
        }
        const_map if *const_map == const_pred => Ok(TermMapType::Constant),
        temp_map if *temp_map == temp_pred => Ok(TermMapType::Template),
        _ => Err(ParseError::Infallible),
    };

    let term_value = trip.o().to_owned().map(|i| i.to_string());

    term_map_type_res.map(|map_type| (map_type, term_value))
}

pub trait TermMapExtractor<T> {
    fn create_constant_map(tm_info: TermMapInfo) -> T;

    fn create_term_map(
        subj_ref: &RcTerm,
        graph_ref: &FastGraph,
    ) -> ExtractorResult<T>;

    fn extract_constant_term_map(map_const: &Term<Rc<str>>) -> T {
        let tm_info = TermMapInfo::from_constant_value(map_const.clone());

        Self::create_constant_map(tm_info)
    }

    fn extract_term_map(
        graph_ref: &FastGraph,
        container_map_subj_ref: &RcTerm,
    ) -> ExtractorResult<T> {
        Self::extract_term_maps(graph_ref, container_map_subj_ref)
            .and_then(|mut vec| vec.pop().ok_or(ParseError::Infallible))
    }
    fn extract_term_maps(
        graph_ref: &FastGraph,
        container_map_subj_ref: &RcTerm,
    ) -> ExtractorResult<Vec<T>> {
        let map_pred = Self::get_map_pred();
        let const_pred = Self::get_const_pred();
        let map_subj_vec_res =
            get_objects(graph_ref, container_map_subj_ref, &map_pred);
        let const_obj_vec_res =
            get_objects(graph_ref, container_map_subj_ref, &const_pred);

        if let Ok(map_subj_vec) = map_subj_vec_res {
            return Ok(map_subj_vec
                .iter()
                .flat_map(|map_subj| {
                    Self::create_term_map(&map_subj, graph_ref)
                })
                .collect());
        } else if let Ok(mut map_const_obj_vec) = const_obj_vec_res {
            return Ok(map_const_obj_vec
                .iter_mut()
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
    fn extract(
        subject_ref: &RcTerm,
        graph_ref: &FastGraph,
    ) -> ExtractorResult<T>;
}

pub trait FromVocab {
    fn to_term(&self) -> RcTerm;
}

impl<'a> FromVocab for PAIR<'a> {
    fn to_term(&self) -> RcTerm {
        Term::new_iri(self.to_string().as_ref()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_MAPPING_STR: &str = r##"
@prefix rr: <http://www.w3.org/ns/r2rml#>.
@prefix rml: <http://semweb.mmlab.be/ns/rml#>.
@prefix ql: <http://semweb.mmlab.be/ns/ql#>.
@prefix transit: <http://vocab.org/transit/terms/>.
@prefix xsd: <http://www.w3.org/2001/XMLSchema#>.
@prefix wgs84_pos: <http://www.w3.org/2003/01/geo/wgs84_pos#>.
@base <http://example.com/ns#>.

<#AirportMapping> a rr:TriplesMap;
  rml:logicalSource [
    rml:source "Airport.csv" ;
    rml:referenceFormulation ql:CSV
  ];
  rr:subjectMap [
    rr:template "http://airport.example.com/{id}";
    rr:class transit:Stop
  ];

  rr:predicateObjectMap [
    rr:predicate transit:route;
    rr:objectMap [
      rml:reference "stop";
      rr:datatype xsd:int
      ]
    ];

  rr:predicateObjectMap [
    rr:predicate wgs84_pos:lat;
    rr:objectMap [
      rml:reference "latitude"
    ]
  ];

  rr:predicateObjectMap [
    rr:predicate wgs84_pos:long;
    rr:objectMap [
      rml:reference "longitude"
    ]
  ]."##;
}
