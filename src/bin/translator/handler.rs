use std::fmt::Debug;

use plangenerator::error::PlanError;
use plangenerator::plan::{Init, Plan};

pub trait FileTranslatorHandler:Debug {
    fn can_handle(&self, file_path: &dyn AsRef<str>) -> bool;
    fn translate(
        &self,
        file_path: &dyn AsRef<str>,
    ) -> Result<Plan<Init>, PlanError>;

    fn handle_file(
        &self,
        file_path: &dyn AsRef<str>,
    ) -> Result<Plan<Init>, PlanError> {
        if !self.can_handle(file_path) {
            return Err(PlanError::GenericError(format!(
                "{:?} do not support handling the file: {}",
                self, 
                file_path.as_ref()
            )));
        }
        self.translate(file_path)
    }
}

