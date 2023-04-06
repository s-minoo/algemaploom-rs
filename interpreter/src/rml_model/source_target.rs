use crate::{IriString, TermString};
// TODO: Implement source and target metadata infos <30-03-23, Min Oo>

#[derive(Debug, Clone)]
pub struct LogicalSource {
    pub identifier:            TermString,
    pub iterator:              Option<String>,
    pub source:                Source,
    pub reference_formulation: IriString,
}

#[derive(Debug, Clone)]
pub struct LogicalTarget {
    pub identifier:    TermString,
    pub compression:   Option<IriString>,
    pub serialization: IriString,
    pub output:        Output,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Source {
    FileInput { path: String },
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
