
use plangenerator::plan::{Plan, PlanNode};
use shexml_interpreter::ShExMLDocument;

use crate::shexml::operators::source::ShExMLSourceTranslator;
use crate::{LanguageTranslator, OperatorTranslator};

mod operators;

pub struct ShExMLTranslator;

impl LanguageTranslator<ShExMLDocument> for ShExMLTranslator {
    fn translate_to_plan(
        model: ShExMLDocument,
    ) -> crate::LanguageTranslateResult {
        let mut plan = Plan::new();

        let source_translator = ShExMLSourceTranslator { document: &model };
        let sources = source_translator.translate();

        todo!()
    }
}
