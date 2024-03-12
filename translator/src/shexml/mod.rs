use plangenerator::plan::Plan;
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
        let document = model.convert_to_indexed();

        let source_translator = ShExMLSourceTranslator {
            document: &document,
        };
        let scident_scexprpair_map = source_translator.translate();

        println!("{:#?}", scident_scexprpair_map);
        for (idx, (source, expr_idents)) in
            scident_scexprpair_map.values().enumerate()
        {
            let sourced_plan = plan.source(source.clone());
        }

        todo!()
    }
}
