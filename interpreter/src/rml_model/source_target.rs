use super::term::IRI;

// TODO: <30-03-23, Min Oo>
// Implement source and target metadata infos
//

#[derive(Debug, Clone)]
pub struct LogicalSource {
    pub identifier:            String,
    pub iterator:              String,
    pub input:                 Input,
    pub reference_formulation: IRI,
}

#[derive(Debug, Clone)]
pub struct LogicalTarget {
    pub identifier:    String,
    pub compression:   Option<IRI>,
    pub serialization: IRI,
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
