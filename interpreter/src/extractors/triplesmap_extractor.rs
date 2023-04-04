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
use crate::rml_model::term_map::{SubjectMap, TriplesMap};

impl Extractor<TriplesMap> for TriplesMap {
    fn extract(
        subject: &TermShared,
        graph: &FastGraph,
    ) -> ExtractorResult<TriplesMap> {
        let ls_term = vocab::rml::PROPERTY::LOGICALSOURCE.to_term();
        let s_map = vocab::r2rml::PROPERTY::SUBJECTMAP.to_term();
        let pom = vocab::r2rml::PROPERTY::PREDICATEOBJECTMAP.to_term();

        let logical_source_subj = get_object(graph, subject, ls_term)?;
        let sm_subj = get_object(graph, subject, s_map)?;
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
            subject_map:    SubjectMap::extract(&sm_subj, graph)?,
            po_maps:        todo!(),
            graph_map:      None,
        };
        todo!()
    }
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
