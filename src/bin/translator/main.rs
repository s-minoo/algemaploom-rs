use std::path::PathBuf;

use clap::Parser;
use interpreter::extractors::io::parse_file;
use interpreter::rmlalgebra::translate_to_algebra;
use plangenerator::error::PlanError;

#[derive(Debug, Clone, Parser)]
#[command(
    name = "RML2Algebra",
    version = "0.1",
    about = "Translates the given RML document into a tree of algebraic mapping operators."
)]

struct Cli {
    /// The RML document to be translated into algebra
    rml_document: PathBuf,

    /// The generated output dot file containing the algebra tree
    #[arg(short, long)]
    output: Option<String>,
}

pub fn main() -> Result<(), PlanError> {
    let args = Cli::parse();

    let document = parse_file(args.rml_document.clone())
        .or_else(|err| Err(PlanError::AuxError(format!("{:?}", err))))?;
    let mut mapping_plan = translate_to_algebra(document)?;

    let output_path: String = args.output.unwrap_or("output.dot".into());

    mapping_plan
        .write(output_path.clone().into())
        .or_else(|err| Err(PlanError::AuxError(format!("{:?}", err))))?;

    let pretty_path = output_path.clone() + ".pretty";

    mapping_plan
        .write_pretty(pretty_path.clone().into())
        .or_else(|err| Err(PlanError::AuxError(format!("{:?}", err))))?;

    println!(
        "The following mapping tree have been translated from {:?} at {:?}",
        args.rml_document, output_path
    );
    println!(
        "The pretty dot file version for visualization is generated at: {:?}",
        pretty_path
    );

    Ok(())
}
