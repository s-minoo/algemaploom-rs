use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use anyhow::Result;
use clap::{command, Parser};
use rml_interpreter::extractors::error::ParseError;
use rml_interpreter::extractors::io::parse_file;

#[derive(Debug, Clone, Parser)]
#[command(
    name = "RMLParser",
    version = "0.1",
    about = "Parses the provided RML document into an internal data model"
)]

struct Cli {
    /// The RML document to be translated into algebra
    rml_document: PathBuf,

    /// The generated output dot file containing the algebra tree
    #[arg(short, long)]
    output: Option<PathBuf>,
}
pub fn main() -> Result<(), ParseError> {
    let arg = Cli::parse();
    let rml_model = parse_file(arg.rml_document)?;

    if let Some(output_path) = arg.output {
        let file = File::create(output_path)?;
        let mut writer = BufWriter::new(file);
        write!(writer, "{:#?}", rml_model)?;
    } else {
        println!("{:#?}", rml_model);
    }

    Ok(())
}
