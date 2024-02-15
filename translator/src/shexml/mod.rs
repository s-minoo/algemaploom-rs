use plangenerator::plan::Plan;
use shexml_interpreter::ShExMLDocument;

use crate::LanguageTranslator;

mod operators;

pub struct ShExMLTranslator;

impl LanguageTranslator<ShExMLDocument> for ShExMLTranslator {
    fn translate_to_plan(
        model: ShExMLDocument,
    ) -> crate::LanguageTranslateResult {
        let mut plan = Plan::new();
        model.sources.iter().for_each(|source| todo!());
        todo!()
    }
}
