mod cli;

use std::path::PathBuf;

use clap::Parser;
use interpreter::extractors::io::parse_file;
use plangenerator::error::PlanError;
use translator::rmlalgebra::translate_to_algebra;
use walkdir::{DirEntry, WalkDir};

use crate::cli::TRANSLATOR_VERSION;

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

fn is_rml_file(entry: &DirEntry) -> bool {
    entry.file_type().is_file()
        && entry
            .file_name()
            .to_str()
            .map(|str_path| str_path.ends_with(".ttl"))
            .unwrap_or(false)
}

pub fn main() -> Result<(), PlanError> {
    let cli = cli::Cli::new();

    let matches = cli.cmd.get_matches();

    if let Some(file_matches) = matches.subcommand_matches("file") {
        let file_path_string: &String =
            file_matches.get_one("RML_DOCUMENT").unwrap();

        let file_path: PathBuf = file_path_string.into();
        let mut output_prefix = Some("output".to_string());
        if let Some(derived_prefix) = file_path.file_stem() {
            let derived_string = derived_prefix.to_string_lossy();
            output_prefix.insert(derived_string.to_string());
        }
        translate_rml_file(
            file_path.to_string_lossy(),
            output_prefix.unwrap(),
        )?;
    } else if let Some(folder_matches) = matches.subcommand_matches("folder") {
        let folder_path_string: &String =
            folder_matches.get_one("FOLDER").unwrap();
        let folder_path: PathBuf = folder_path_string.into();
        let rml_files = WalkDir::new(folder_path)
            .into_iter()
            .filter_entry(|entry| is_rml_file(entry))
            .flatten();

        for rml_file in rml_files {
            let file = rml_file.path();

            let output_dir = file
                .parent()
                .map_or("".to_string(), |p| p.to_string_lossy().to_string());
            let output_prefix =
                output_dir + &file.file_stem().unwrap().to_string_lossy();
            let _ = translate_rml_file(file.to_string_lossy(), output_prefix);
        }
    }
    Ok(())
}

fn translate_rml_file<F: AsRef<str>, O: AsRef<str>>(
    file: F,
    output_prefix: O,
) -> Result<(), PlanError> {
    let document = parse_file(file.as_ref().into())
        .or_else(|err| Err(PlanError::GenericError(format!("{:?}", err))))?;

    let output_prefix = output_prefix.as_ref().to_string();
    let mut mapping_plan = translate_to_algebra(document)?;
    let full_path = output_prefix.clone() + ".dot";
    let _ = mapping_plan
        .write(full_path.clone().into())
        .or_else(|err| Err(PlanError::GenericError(format!("{:?}", err))));
    let pretty_path = output_prefix + "_pretty.dot";

    let _ = mapping_plan
        .write_pretty(pretty_path.clone().into())
        .or_else(|err| Err(PlanError::GenericError(format!("{:?}", err))));

    println!(
        "The following mapping tree have been translated from {:?} at {:?}",
        file.as_ref(),
        full_path
    );
    println!(
        "The pretty dot file version for visualization is generated at: {:?}",
        pretty_path
    );
    Ok(())
}

