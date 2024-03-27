use plangenerator::error::PlanError;
use plangenerator::plan::{Init, Plan};
use translator::LanguageTranslator;

use crate::handler::FileTranslatorHandler;

#[derive(Debug, Clone)]
pub struct ShExMLFileHandler;

impl FileTranslatorHandler for ShExMLFileHandler {
    fn translate(
        &self,
        file_path: &dyn AsRef<str>,
    ) -> Result<Plan<Init>, PlanError> {
        let shexml_document = shexml_interpreter::parse_file(
            file_path.as_ref(),
        )
        .map_err(|shex_err| {
            PlanError::GenericError(format!(
                "Something went wrong while parsing shexml: \n {:?}",
                shex_err
            ))
        })?;

        translator::shexml::ShExMLTranslator::translate_to_plan(shexml_document)
    }

    fn supported_extension(&self) -> String {
        "shexml".to_string()
    }
}
