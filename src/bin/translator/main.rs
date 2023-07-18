use std::fs::File;
use std::path::PathBuf;

use clap::Parser;
use interpreter::extractors::io::parse_file;
use interpreter::extractors::ExtractorResult;
use interpreter::rmlalgebra::translate_to_algebra;

#[derive(Debug, Clone, Parser)]
#[command(
    name = "RML2Algebra",
    version = "0.1",
    about = "Translates the given RML document into a tree of algebraic mapping operators."
)]
struct Cli {

    /// The RML document to be translated into algebra
    rml_document: PathBuf,

    /// The generated output json file containing the algebra tree
    #[arg(short, long)]
    output: Option<PathBuf>,
}

pub fn main() -> ExtractorResult<()> {
    let args = Cli::parse();

    let document = parse_file(args.rml_document.clone())?;
    let operators = translate_to_algebra(document);

    let output_path = args.output.unwrap_or("output.json".into());

    let output = File::create(output_path.clone())?;

    serde_json::to_writer_pretty(output, &operators);

    println!(
        "The following mapping tree have been translated from {:?} at {:?}",
        args.rml_document, output_path
    );
    
    let separator = (0..20).into_iter().fold("".to_string(), |acc,_| acc + "\"");
    
    println!("{}", separator);
    println!("{:#?}", operators);

    Ok(())
}
