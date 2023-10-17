use std::collections::HashMap;
use std::hash::Hash;

use operator::formats::DataFormat;
use operator::{IOType, Target};
use sophia_api::term::TTerm;
use sophia_term::iri::Iri;
use vocab::ToString;

use crate::extractors::FromVocab;
use crate::IriString;

#[derive(Debug, Clone, Eq)]
pub struct LogicalSource {
    pub identifier: String,
    pub iterator: Option<String>,
    pub source: Source,
    pub reference_formulation: IriString,
}
impl PartialEq for LogicalSource {
    fn eq(&self, other: &Self) -> bool {
        self.iterator == other.iterator
            && self.source == other.source
            && self.reference_formulation == other.reference_formulation
    }
}

impl From<LogicalSource> for operator::Source {
    fn from(val: LogicalSource) -> Self {
        let source_type = match &val.source {
            Source::FileInput { path: _ } => IOType::File,

            // TODO: Determine the IOType for CSVW from the specified URL! <27-09-23, Min Oo> //
            Source::CSVW {
                url: _,
                parse_config: _,
            } => IOType::File,
        };

        let data_format = match &val.reference_formulation.value().to_string() {
            p if *p == vocab::query::CLASS::CSV.to_string() => DataFormat::CSV,
            p if *p == vocab::query::CLASS::JSONPATH.to_string() => {
                DataFormat::JSON
            }
            p if *p == vocab::query::CLASS::XPATH.to_string() => {
                DataFormat::XML
            }
            p => panic!("Data format not supported {} ", p),
        };

        let reference_iterators = match &val.iterator {
            Some(iter) => vec![iter.to_owned()],
            None => vec![],
        };

        operator::Source {
            config: source_config_map(&val),
            reference_iterators,
            source_type,
            data_format,
        }
    }
}

#[derive(Debug, Clone)]
pub enum FileMode {
    Append,
    Overwrite,
}

#[derive(Debug, Clone)]
pub enum Output {
    FileOutput { path: String, mode: FileMode },
}

pub fn default_file_output(path: String) -> Output {
    Output::FileOutput {
        path,
        mode: FileMode::Overwrite,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicalTarget {
    pub identifier: String,
    pub compression: Option<IriString>,
    pub serialization: IriString,
    pub output_type: IOType,
    pub config: HashMap<String, String>,
}

impl Default for LogicalTarget {
    fn default() -> Self {
        Self {
            identifier: String::from("default"),
            compression: Default::default(),
            serialization: Iri::new(
                vocab::formats::CLASS::NTRIPLES.to_string(),
            )
            .unwrap(),
            output_type: Default::default(),
            config: Default::default(),
        }
    }
}

impl Hash for LogicalTarget {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.identifier.hash(state);
    }
}

fn serialization_to_dataformat(serialization: &IriString) -> DataFormat {
    match serialization.to_owned() {
        ser_iri_string
            if ser_iri_string == vocab::formats::CLASS::TURTLE.to_rcterm() =>
        {
            DataFormat::TTL
        }
        ser_iri_string
            if ser_iri_string
                == vocab::formats::CLASS::NTRIPLES.to_rcterm() =>
        {
            DataFormat::NTriples
        }
        ser_iri_string
            if ser_iri_string == vocab::formats::CLASS::JSONLD.to_rcterm() =>
        {
            DataFormat::JSONLD
        }
        ser_iri_string
            if ser_iri_string == vocab::formats::CLASS::NQUADS.to_rcterm() =>
        {
            DataFormat::NQuads
        }

        _ => DataFormat::NTriples,
    }
}

impl From<&LogicalTarget> for operator::Target {
    fn from(val: &LogicalTarget) -> Self {
        let mut configuration = HashMap::new();

        if let Some(comp_iri) = val.compression.as_ref() {
            configuration.insert(
                "compresssion".to_string(),
                comp_iri.value().to_string(),
            );
        }
        configuration.extend(val.config.clone());

        let data_format = serialization_to_dataformat(&val.serialization);
        Target {
            configuration,
            data_format,
            target_type: val.output_type.clone(),
        }
    }
}

impl From<LogicalTarget> for operator::Target {
    fn from(val: LogicalTarget) -> Self {
        (&val).into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Source {
    CSVW {
        url: String,
        parse_config: HashMap<String, String>,
    },
    FileInput {
        path: String,
    },
}

impl From<Source> for HashMap<String, String> {
    fn from(val: Source) -> Self {
        let mut map = HashMap::new();
        match val {
            Source::FileInput { path } => {
                map.insert("path".to_string(), path);
            }
            Source::CSVW { url, parse_config } => {
                map.insert("url".to_string(), url);
                map.extend(parse_config);
            }
        };

        map
    }
}

fn source_config_map(ls: &LogicalSource) -> HashMap<String, String> {
    let mut map = HashMap::new();

    map.insert("identifier".to_string(), ls.identifier.to_string());

    if let Some(iter) = &ls.iterator {
        map.insert("iterator".to_string(), iter.to_owned());
    }

    let source_map: HashMap<String, String> = ls.source.clone().into();

    map.extend(source_map);

    map
}
