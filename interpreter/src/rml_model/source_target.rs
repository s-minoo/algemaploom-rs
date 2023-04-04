use sophia_term::iri::Iri;

use crate::{LiteralString, IriString};
// TODO: <30-03-23, Min Oo>
// Implement source and target metadata infos
//

#[derive(Debug, Clone)]
pub struct LogicalSource {
    pub identifier:            String,
    pub iterator:              String,
    pub input:                 Input,
    pub reference_formulation: IriString,
}

#[derive(Debug, Clone)]
pub struct LogicalTarget {
    pub identifier:    String,
    pub compression:   Option<IriString>,
    pub serialization: IriString,
    pub output:        Output,
}

#[derive(Debug, Clone)]
pub enum Input {
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
