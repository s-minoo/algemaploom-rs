use std::{collections::HashSet, str::FromStr};

use lazy_static::lazy_static;
use serde_json::Number;
use sophia_api::term::TTerm;
use sophia_term::RcTerm;
use vocab::ToString;

use super::ExtractorResult;
use crate::extractors::{error::ParseError, FromVocab};

lazy_static! {
    static ref NUMBER_IRIS: HashSet<String> = HashSet::from([
        vocab::xsd::TYPE::XSD_POSITIVE_INTEGER.to_string(),
        vocab::xsd::TYPE::XSD_INT.to_string(),
        vocab::xsd::TYPE::XSD_INTEGER.to_string(),
        vocab::xsd::TYPE::XSD_LONG.to_string(),
        vocab::xsd::TYPE::XSD_DOUBLE.to_string(),
    ]);
}

pub fn term_to_value(term: &RcTerm) -> ExtractorResult<serde_json::Value> {
    match term.kind(){
        sophia_api::term::TermKind::Literal => Ok(()),
        _ => Err(ParseError::GenericError(
            "Blank nodes and variables cannot be converted to serde values"
                .to_string(),
        )),
    }?;

    //Term kind is literal so start parsing the datatypes
    let raw_val = term.value().to_string();
    let data_type_iri = term.datatype().unwrap();
    match data_type_iri {
        iri if iri == vocab::xsd::TYPE::XSD_BOOLEAN.to_rcterm() => {
            Ok(serde_json::Value::Bool(raw_val.parse().unwrap()))
        }
        iri if NUMBER_IRIS.contains(iri.value().as_ref()) => {
            Ok(serde_json::Value::Number(Number::from_str(&raw_val)?))
        }
        _ => Err(ParseError::GenericError(format!(
            "Parsing data type {} is not supported yet",
            data_type_iri
        ))),
    }
}
