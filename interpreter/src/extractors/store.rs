use std::fmt::Display;

use sophia_api::graph::Graph;
use sophia_api::term::TTerm;
use sophia_api::triple::Triple;
use sophia_inmem::graph::FastGraph;

use super::error::ParseError;
use super::RcTerm;

pub fn get_subject<T>(
    graph: &FastGraph,
    pred: &T,
    obj: &T,
) -> Result<RcTerm, ParseError>
where
    T: TTerm + ?Sized + Display,
{
    graph
        .triples_with_po(pred, obj)
        .next()
        .map(|trip_res| trip_res.map(|trip| trip.o().to_owned()).unwrap())
        .ok_or(ParseError::GenericError(format!(
            "Subject not found in graph with obj {} and pred {}",
            pred, obj
        )))
}
pub fn get_objects<T>(
    graph: &FastGraph,
    subject: &T,
    pred: &T,
) -> Result<Vec<RcTerm>, ParseError>
where
    T: TTerm + ?Sized + Display,
{
    Ok(graph
        .triples_with_sp(subject, pred)
        .filter_map(|trip_res| trip_res.ok().map(|trip| trip.o().to_owned()))
        .collect())
}
pub fn get_object<T>(
    graph: &FastGraph,
    subject: &T,
    pred: &T,
) -> Result<RcTerm, ParseError>
where
    T: TTerm + ?Sized + Display,
{
    let mut objects = get_objects(graph, subject, pred)?;

    objects.pop().ok_or(ParseError::GenericError(format!(
        "Object not found in graph with subj {} and pred {}",
        subject, pred
    )))
}
