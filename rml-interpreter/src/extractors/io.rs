use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use sophia_api::triple::stream::TripleSource;
use sophia_inmem::graph::FastGraph;
use sophia_turtle::parser::turtle;

use super::error::ParseError;
use super::triplesmap_extractor::extract_triples_maps;
use super::ExtractorResult;
use crate::rml_model::Document;

pub fn load_graph_bread(buf_read: impl BufRead) -> ExtractorResult<FastGraph> {
    match turtle::parse_bufread(buf_read).collect_triples() {
        Ok(it) => Ok(it),
        Err(err) => {
            Err(ParseError::GenericError(format!(
                "Something went wrong with sophia's turtle parsing: {}",
                err
            )))
        }
    }
}

pub fn load_graph_str(input_str: &str) -> ExtractorResult<FastGraph> {
    match turtle::parse_str(input_str).collect_triples() {
        Ok(it) => Ok(it),
        Err(err) => {
            Err(ParseError::GenericError(format!(
                "Something went wrong with sophia's turtle parsing: {}",
                err
            )))
        }
    }
}

pub fn parse_str(input_str: &str) -> ExtractorResult<Document> {
    let graph = load_graph_str(input_str)?;
    let triples_maps = extract_triples_maps(&graph)?;
    Ok(Document { triples_maps })
}

pub fn parse_file(path: PathBuf) -> ExtractorResult<Document> {
    if let Some(ext) = path.extension() {
        if ext != "ttl" {
            return Err(ParseError::ExtensionError(format!(
                "Extension does not exist {}",
                ext.to_str().unwrap()
            )));
        }

        let buf_read = BufReader::new(File::open(path)?);
        let triples_maps = extract_triples_maps(&load_graph_bread(buf_read)?)?;
        return Ok(Document { triples_maps });
    }

    Err(ParseError::IOErrorStr(format!(
        "File can't be read {}",
        path.to_str().unwrap()
    )))
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::test_case;

    #[test]
    fn one_tm_test() -> ExtractorResult<()> {
        let path = PathBuf::from(test_case!("sample_mapping.ttl"));
        let parsed_res = parse_file(path)?;

        // One TriplesMap should be parsed
        assert!(parsed_res.triples_maps.len() == 1);

        Ok(())
    }

    #[test]
    fn multiple_tm_test() {
        let path = PathBuf::from(test_case!("multiple_tm.ttl"));
        let parsed_res = parse_file(path);

        assert!(parsed_res.is_ok());
        // One TriplesMap should be parsed
        assert!(parsed_res.unwrap().triples_maps.len() == 2);
    }
}
