use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use operator::Function;
use plangenerator::plan::{Plan, Processed, RcRefCellPlan};
use shexml_interpreter::{
    get_quads_from_same_source, IndexedShExMLDocument, ShExMLDocument,
    ShExMLQuads,
};

use crate::shexml::operators::extend;
use crate::shexml::operators::source::ShExMLSourceTranslator;
use crate::{LanguageTranslator, OperatorTranslator};

mod operators;

pub struct ShExMLTranslator;

impl LanguageTranslator<ShExMLDocument> for ShExMLTranslator {
    fn translate_to_plan(
        model: ShExMLDocument,
    ) -> crate::LanguageTranslateResult {
        let mut plan = Plan::new();
        let indexed_document = model.convert_to_indexed();

        let source_translator = ShExMLSourceTranslator {
            document: &indexed_document,
        };

        let scidentkey_sourcedplan_exprident_pairval_map: HashMap<
            String,
            (RcRefCellPlan<Processed>, Vec<String>),
        > = source_translator
            .translate()
            .into_iter()
            .map(|(key, value)| {
                (key, (Rc::new(RefCell::new(plan.source(value.0))), value.1))
            })
            .collect();

        for (source_ident, (sourced_plan, expr_idents)) in
            scidentkey_sourcedplan_exprident_pairval_map.iter()
        {
            let expr_idents_hashset =
                expr_idents.iter().map(|ident| ident.as_str()).collect();

            //filter out quads that could be generated from the same source

            let filtered_same_source_quads = get_quads_from_same_source(
                indexed_document.graph_shapes.values(),
                expr_idents_hashset,
            );

            add_non_join_related_op(
                &indexed_document,
                &filtered_same_source_quads,
                sourced_plan.clone(),
            );
        }

        todo!()
    }
}

fn add_non_join_related_op(
    doc: &IndexedShExMLDocument,
    quads: &ShExMLQuads<'_>,
    sourced_plan: RcRefCellPlan<Processed>,
) -> Plan<Processed> {
    let extended_plan =
        add_extend_op_from_quads(doc, quads, sourced_plan.clone());

    todo!()
}

fn add_extend_op_from_quads(
    doc: &IndexedShExMLDocument,
    quads: &ShExMLQuads<'_>,
    sourced_plan: RcRefCellPlan<Processed>,
) -> RcRefCellPlan<Processed> {
    let mut extendvar_func_pairs: Vec<(String, Function)> = Vec::new();
    let mut expression_stmts_map = &doc.expression_stmts;
    for (subj, _, obj, _) in quads {
        let mut expr_ident_set = subj.expression.extract_expr_idents();
        expr_ident_set.extend(obj.expression.extract_expr_idents());

        for expr_ident in expr_ident_set {
            if let Some(expresion_stmt) = expression_stmts_map.get(expr_ident) {
                let concate_extend_pairs = extend::translate_concatenate(
                    expr_ident,
                    &doc.iterators,
                    &expresion_stmt.expr_enum,
                );

                extendvar_func_pairs.extend(concate_extend_pairs);
            }
        }
    }

    todo!()
}

#[cfg(test)]
mod tests {
    use plangenerator::error::PlanError;

    use super::*;
    use crate::test_case;

    #[ignore]
    #[test]
    fn translate_to_plan_test() -> Result<(), PlanError> {
        let input_shexml = test_case!("shexml/sample.shexml");
        let shexml_document =
            shexml_interpreter::parse_file(input_shexml).unwrap();

        ShExMLTranslator::translate_to_plan(shexml_document)?;
        Ok(())
    }
}
