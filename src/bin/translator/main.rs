mod cli;
mod handler;
mod rml;
mod shexml;
mod util;

use std::path::PathBuf;

use colored::Colorize;
use handler::FileTranslatorHandler;
use meamer_rs::logger::init_logger;
use plangenerator::error::PlanError;
use util::serialize_and_log_msg;
use walkdir::WalkDir;

use crate::rml::RMLFileHandler;
use crate::shexml::ShExMLFileHandler;

fn init_handlers() -> Vec<Box<dyn FileTranslatorHandler>> {
    vec![Box::new(RMLFileHandler), Box::new(ShExMLFileHandler)]
}

pub fn main() -> Result<(), PlanError> {
    let cli = cli::Cli::new();
    init_logger().map_err(|err| PlanError::GenericError(err.to_string()))?;

    let matches = cli.cmd.get_matches();
    let mut err_vec = Vec::new();
    let handlers = init_handlers();

    if let Some(file_matches) = matches.subcommand_matches("file") {
        let file_path_string: &String =
            file_matches.get_one("DOCUMENT").unwrap();

        let file_path: PathBuf = file_path_string.into();
        let mut output_prefix = Some("output".to_string());
        if let Some(derived_prefix) = file_path.file_stem() {
            let derived_string = derived_prefix.to_string_lossy();
            let _ = output_prefix.insert(derived_string.to_string());
        }

        process_one_file(&handlers, file_path, &mut err_vec, output_prefix)?;
    } else if let Some(folder_matches) = matches.subcommand_matches("folder") {
        let folder_path_string: &String =
            folder_matches.get_one("FOLDER").unwrap();
        let folder_path: PathBuf = folder_path_string.into();
        let files = WalkDir::new(folder_path)
            .max_depth(4)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|dentry| dentry.file_type().is_file())
            .filter(|file| {
                handlers.iter().any(|handler| {
                    handler.can_handle(&file.path().to_string_lossy())
                })
            });

        for file in files {
            let input_path = file.path();

            let output_dir = input_path
                .parent()
                .map_or("".to_string(), |p| p.to_string_lossy().to_string());
            let output_prefix = output_dir
                + "/"
                + &input_path.file_stem().unwrap().to_string_lossy();

            process_one_file(
                &handlers,
                input_path.to_path_buf(),
                &mut err_vec,
                Some(output_prefix),
            )?;
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

fn process_one_file(
    handlers: &[Box<dyn FileTranslatorHandler>],
    file_path: PathBuf,
    err_vec: &mut Vec<(String, PlanError)>,
    output_prefix: Option<String>,
) -> Result<(), PlanError> {
    let (generated_plans, generated_errors_res): (Vec<_>, Vec<_>) = handlers
        .iter()
        .map(|handler| handler.translate(&file_path.to_string_lossy()))
        .partition(|plan| plan.is_ok());
    if generated_plans.is_empty() {
        let generated_errors = generated_errors_res
            .into_iter()
            .flat_map(|pe| pe.err())
            .map(|err| (file_path.to_string_lossy().to_string(), err));

        err_vec.extend(generated_errors);
    } else {
        for mut plan in generated_plans.into_iter().flat_map(|p_res| p_res.ok())
        {
            serialize_and_log_msg(
                output_prefix.clone().unwrap(),
                &mut plan,
                file_path.to_string_lossy(),
            )?;
        }
    };
    Ok(())
}
