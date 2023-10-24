mod cli;

use std::path::PathBuf;

use colored::Colorize;
use interpreter::extractors::io::parse_file;
use plangenerator::error::PlanError;
use translator::rmlalgebra::translate_to_algebra;
use walkdir::{DirEntry, WalkDir};

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
    let mut err_vec = Vec::new();

    if let Some(file_matches) = matches.subcommand_matches("file") {
        let file_path_string: &String =
            file_matches.get_one("RML_DOCUMENT").unwrap();

        let file_path: PathBuf = file_path_string.into();
        let mut output_prefix = Some("output".to_string());
        if let Some(derived_prefix) = file_path.file_stem() {
            let derived_string = derived_prefix.to_string_lossy();
            let _ = output_prefix.insert(derived_string.to_string());
        }
        if let Err(err) = translate_rml_file(
            file_path.to_string_lossy(),
            output_prefix.unwrap(),
        ) {
            err_vec.push((file_path.to_string_lossy().to_string(), err));
        }
    } else if let Some(folder_matches) = matches.subcommand_matches("folder") {
        let folder_path_string: &String =
            folder_matches.get_one("FOLDER").unwrap();
        let folder_path: PathBuf = folder_path_string.into();
        let rml_files = WalkDir::new(folder_path)
            .max_depth(4)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| is_rml_file(entry));

        for rml_file in rml_files {
            let file = rml_file.path();

            let output_dir = file
                .parent()
                .map_or("".to_string(), |p| p.to_string_lossy().to_string());
            let output_prefix =
                output_dir + "/" + &file.file_stem().unwrap().to_string_lossy();
            if let Err(err) =
                translate_rml_file(file.to_string_lossy(), output_prefix)
            {
                err_vec.push((file.to_string_lossy().to_string(), err));
            }
        }
    }

    err_vec.iter().for_each(|(file, err)| {
        eprintln!(
            "{}: Errored while translating {}",
            "Error".red(),
            file.yellow()
        );
        let err_string = format!("{}", err);
        eprintln!("{}\n", err_string.red());
    });

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
        .or_else(|err| Err(PlanError::GenericError(format!("{:?}", err))))?;
    let pretty_path = output_prefix + "_pretty.dot";

    let _ = mapping_plan
        .write_pretty(pretty_path.clone().into())
        .or_else(|err| Err(PlanError::GenericError(format!("{:?}", err))))?;

    println!(
        "{}: Translating {}",
        "Success".green(),
        file.as_ref().yellow(),
    );
    println!("Generated dot file: {}", full_path.yellow());
    println!(
        "The pretty dot file version for visualization is: {}\n",
        pretty_path.yellow()
    );
    Ok(())
}
