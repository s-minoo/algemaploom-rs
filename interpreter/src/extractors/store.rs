use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use sophia_api::graph::Graph;
use sophia_api::triple::stream::TripleSource;
use sophia_api::triple::Triple;
use sophia_inmem::graph::FastGraph;
use sophia_turtle::parser::turtle;

use super::error::ParseError;
use super::triplesmap_extractor::extract_triples_maps;
use super::{TermShared, TermString};
use crate::rml_model::term_map::TriplesMap;

pub fn get_subject(
    graph: &FastGraph,
    pred: &TermString,
    obj: &TermShared,
) -> Result<TermShared, ParseError> {
    graph
        .triples_with_po(pred, obj)
        .next()
        .map(|trip_res| trip_res.map(|trip| trip.o().to_owned()).unwrap())
        .ok_or(ParseError::GenericError(format!(
            "Subject not found in graph with obj {} and pred {}",
            pred, obj
        )))
}
pub fn get_objects(
    graph: &FastGraph,
    subject: &TermShared,
    pred: &TermString,
) -> Result<Vec<TermShared>, ParseError> {
    Ok(graph
        .triples_with_sp(subject, pred)
        .filter_map(|trip_res| trip_res.ok().map(|trip| trip.o().to_owned()))
        .collect())
}
pub fn get_object(
    graph: &FastGraph,
    subject: &TermShared,
    pred: &TermString,
) -> Result<TermShared, ParseError> {
    let mut objects = get_objects(graph, subject, pred)?;

    objects.pop().ok_or(ParseError::GenericError(format!(
        "Object not found in graph with subj {} and pred {}",
        subject, pred
    )))
}

pub fn parse_bread(buf_read: impl BufRead) -> Result<FastGraph, ParseError> {
    match turtle::parse_bufread(buf_read).collect_triples() {
        Ok(it) => return Ok(it),
        Err(err) => {
            return Err(ParseError::GenericError(format!(
                "Something went wrong with sophia's turtle parsing: {}",
                err
            )))
        }
    }
}

pub fn parse_file(path: PathBuf) -> Result<Vec<TriplesMap>, ParseError> {
    if let Some(ext) = path.extension() {
        if ext != "ttl" {
            return Err(ParseError::ExtensionError(format!(
                "Extension does not exist {}",
                ext.to_str().unwrap()
            )));
        }

        let buf_read = BufReader::new(File::open(path)?);
        return extract_triples_maps(&parse_bread(buf_read)?);
    }

    Err(ParseError::IOErrorStr(format!(
        "File can't be read {}",
        path.to_str().unwrap()
    )))
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

    #[test]
    fn store_test() {
        let buf_read = BufReader::new(TEST_MAPPING_STR.as_bytes());

        assert!(parse_bread(buf_read).is_ok());
    }
}
