use std::path::PathBuf;

use plangenerator::error::PlanError;
use plangenerator::plan::{Init, Plan};
use rml_interpreter::extractors::io::parse_file;
use translator::rmlalgebra::OptimizedRMLDocumentTranslator;
use translator::LanguageTranslator;

use crate::handler::FileTranslatorHandler;

#[derive(Debug)]
pub struct RMLFileHandler;

impl FileTranslatorHandler for RMLFileHandler {
    fn translate(
        &self,
        file_path: &dyn AsRef<str>,
    ) -> Result<Plan<Init>, PlanError> {
        let document = parse_file(file_path.as_ref().into())
            .map_err(|err| PlanError::GenericError(format!("{:?}", err)))?;

        OptimizedRMLDocumentTranslator::translate_to_plan(document)
    }

    fn supported_extension(&self) -> String {
        "ttl".to_string()
    }
}
