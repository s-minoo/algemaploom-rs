use std::collections::HashSet;

use sophia_api::graph::Graph;
use sophia_api::triple::Triple;
use sophia_inmem::graph::FastGraph;
use sophia_term::Term;
use vocab::ToString;

use super::error::ParseError;
use super::{Extractor, ExtractorResult, TermShared, TermString};
use crate::extractors::store::get_object;
use crate::extractors::FromVocab;
use crate::rml_model::source_target::LogicalSource;
use crate::rml_model::term_map::{
    ConstantTermMapInfo, SubjectMap, TermMapInfo, TriplesMap,
};
use crate::IriString;

impl Extractor<TriplesMap> for TriplesMap {
    fn extract(
        subject: &TermShared,
        graph: &FastGraph,
    ) -> ExtractorResult<TriplesMap> {
        let ls_term = vocab::rml::PROPERTY::LOGICALSOURCE.to_term();
        let pom = vocab::r2rml::PROPERTY::PREDICATEOBJECTMAP.to_term();

        let logical_source_subj = get_object(graph, subject, &ls_term)?;
        let poms: Vec<_> = graph
            .triples_with_sp(subject, &pom)
            .filter_map(|triples| triples.ok())
            .map(|triple| triple.o().to_owned())
            .collect();

        TriplesMap {
            identifier:     subject.to_string(),
            logical_source: LogicalSource::extract(
                &logical_source_subj,
                graph,
            )?,
            subject_map:    extract_subject_map(graph, subject)?,
            po_maps:        todo!(),
            graph_map:      None,
        };
        todo!()
    }
}
fn extract_subject_map(
    graph_ref: &FastGraph,
    tmmap_subj_ref: &TermShared,
) -> ExtractorResult<SubjectMap> {
    let s_map = vocab::r2rml::PROPERTY::SUBJECTMAP.to_term();
    let s_const = vocab::r2rml::PROPERTY::SUBJECT.to_term();

    let sm_subj_res = get_object(graph_ref, tmmap_subj_ref, &s_map);
    let sm_const_obj_res = get_object(graph_ref, tmmap_subj_ref, &s_const);

    if let Ok(sm_subj) = sm_subj_res {
        return SubjectMap::extract(&sm_subj, graph_ref);
    } else if let Ok(sm_const_subj) = sm_const_obj_res {
        let map = sm_const_subj.map(|i| i.to_string());
        let identifier: IriString = map.clone().try_into()?;

        let tm_info = TermMapInfo::constant_term_map(
            identifier,
            // TODO:  <04-04-23, Min Oo> //
            // Implement the logical targets parsing properly!!
            HashSet::new(),
            map,
        );

        return Ok(SubjectMap {
            tm_info,
            classes: Vec::new(),
        });
    }

    Err(ParseError::GenericError(format!(
        "TriplesMap {} has no subject map!",
        tmmap_subj_ref
    )))
}

pub fn extract_triples_maps(
    graph: &FastGraph,
) -> Result<Vec<TriplesMap>, ParseError> {
    let ptype: TermString =
        Term::new_iri(vocab::rdf::PROPERTY::TYPE.to_string())?;
    let otm: TermString =
        Term::new_iri(vocab::r2rml::CLASS::TRIPLESMAP.to_string())?;

    Ok(graph
        .triples_with_po(&ptype, &otm)
        .filter_map(|triple| triple.ok())
        .filter_map(|triple| TriplesMap::extract(triple.s(), graph).ok())
        .collect())
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
