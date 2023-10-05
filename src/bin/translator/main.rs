mod cli;

use std::path::PathBuf;

use clap::Parser;
use interpreter::extractors::io::parse_file;
use plangenerator::error::PlanError;
use translator::rmlalgebra::translate_to_algebra;

#[derive(Debug, Clone, Parser)]
#[command(
    name = "RML2Algebra",
    version = "0.1",
    about = "Translates the given RML document into a tree of algebraic mapping operators."
)]

struct Cli {
    /// The RML document to be translated into algebra
    #[arg(short, long)]
    rml_document: Option<PathBuf>,

    /// RML Workspace folder from which all RML documents
    /// will be translated into algebra
    #[arg(short, long)]
    rml_workspace: Option<PathBuf>,

    /// The generated output dot file containing the algebra tree
    #[arg(short, long)]
    output: Option<String>,
}

pub fn main() -> Result<(), PlanError> {
    let args = Cli::parse();

    let document = parse_file(args.rml_document.as_ref().unwrap().clone())
        .or_else(|err| Err(PlanError::GenericError(format!("{:?}", err))))?;
    let mut mapping_plan = translate_to_algebra(document)?;

    let output_path: String = args.output.unwrap_or("output.dot".into());

    mapping_plan
        .write(output_path.clone().into())
        .or_else(|err| Err(PlanError::GenericError(format!("{:?}", err))))?;

    let pretty_path = output_path.clone() + ".pretty";

    mapping_plan
        .write_pretty(pretty_path.clone().into())
        .or_else(|err| Err(PlanError::GenericError(format!("{:?}", err))))?;

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
