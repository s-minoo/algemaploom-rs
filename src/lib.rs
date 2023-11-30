use colored::Colorize;
use plangenerator::error::PlanError;
use rml_interpreter::extractors::io::{parse_file, parse_str};
use translator::rmlalgebra::translate_to_algebra;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub fn translate_rml_str(rml_doc: &str) -> Result<String, JsValue> {
    let document = parse_str(rml_doc)
        .or_else(|err| Err(format!("Error parsing RML: {:?}", err)))?;
    let mapping_plan = translate_to_algebra(document).or_else(|err| {
        Err(format!("Error translating to algebra: {:?}", err))
    })?;

    Ok(mapping_plan.to_string())
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
