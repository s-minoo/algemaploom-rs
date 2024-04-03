use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use log::{debug, trace};
use operator::{Extend, Function, Rename, Serializer, Target};
use plangenerator::error::PlanError;
use plangenerator::plan::{Plan, Processed, RcRefCellPlan, Serialized, Sunk};
use shexml_interpreter::{
    IndexedShExMLDocument, Object, PrefixNameSpace, ShExMLDocument, ShapeIdent,
    Subject,
};

use self::util::IndexVariableTerm;
use crate::shexml::operators::source::ShExMLSourceTranslator;
use crate::shexml::operators::{extend, rename};
use crate::shexml::util::{
    get_quads_from_same_source, variablelize_quads, ShExMLQuads,
};
use crate::{LanguageTranslator, OperatorTranslator};

mod operators;
#[cfg(test)]
mod tests;
mod util;

pub struct ShExMLTranslator;

impl LanguageTranslator<ShExMLDocument> for ShExMLTranslator {
    fn translate_to_plan(
        model: ShExMLDocument,
    ) -> crate::LanguageTranslateResult {
        let mut plan = Plan::new();
        debug!("Indexing shexml document");
        let indexed_document = model.convert_to_indexed();

        trace!("Indexed document: {:#?}", indexed_document);
        let source_translator = ShExMLSourceTranslator {
            document: &indexed_document,
        };

        debug!("Translating all source operators");
        let scidentkey_sourcedplan_exprident_pairval_map: HashMap<
            String,
            (RcRefCellPlan<Processed>, Vec<String>),
        > = source_translator
            .translate()?
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
                &indexed_document, 
                indexed_document.graph_shapes.values(),
                expr_idents_hashset,
            );

            debug!(
                "Adding non join related ops for source: {:?}",
                source_ident
            );
            trace!("Quads: {:#?}", filtered_same_source_quads);
            add_non_join_related_op(
                &indexed_document,
                &filtered_same_source_quads,
                sourced_plan.clone(),
            )?;
        }

        // TODO: Also try to handle joins across different sources in ShExML  <21-03-24, Min Oo> //

        Ok(plan)
    }
}

fn add_non_join_related_op(
    doc: &IndexedShExMLDocument,
    quads: &ShExMLQuads<'_>,
    sourced_plan: RcRefCellPlan<Processed>,
) -> Result<Plan<Sunk>, PlanError> {
    debug!("Variabelizing quads");
    let variablized_terms = variablelize_quads(quads);
    let mut renamed_extended_plan = add_rename_extend_op_from_quads(
        doc,
        quads,
        sourced_plan.clone(),
        &variablized_terms,
    )?;

    let mut serialized_plan = add_serializer_op_from_quads(
        doc,
        quads,
        &mut renamed_extended_plan,
        &variablized_terms,
    )?;

    serialized_plan.sink(&Target {
        configuration: HashMap::new(),
        target_type:   operator::IOType::StdOut,
        data_format:   operator::formats::DataFormat::NQuads,
    })
}

fn add_serializer_op_from_quads(
    doc: &IndexedShExMLDocument,
    quads: &ShExMLQuads<'_>,
    extended_plan: &mut Plan<Processed>,
    variablized_terms: &IndexVariableTerm<'_>,
) -> Result<Plan<Serialized>, PlanError> {
    let mut bgp_patterns = Vec::new();
    for (subj, pred, obj, graph) in quads {
        let subj_variable =
            variablized_terms.subject_variable_index.get(*subj).unwrap();
        let obj_variable =
            variablized_terms.object_variable_index.get(*obj).unwrap();

        if let Some(pred_prefix_value) =
            doc.prefixes.get(&pred.prefix.to_string())
        {
            let pred_prefix_uri = &pred_prefix_value.uri;
            let graph_value = if graph.prefix == PrefixNameSpace::BasePrefix {
                "".to_string()
            } else {
                let graph_prefix_uri =
                    &doc.prefixes.get(&graph.prefix.to_string()).unwrap().uri;
                format!("{}{}", graph_prefix_uri, graph.local)
            };

            let single_bgp = format!(
                "?{} <{}{}> ?{} {}.",
                subj_variable,
                pred_prefix_uri,
                pred.local,
                obj_variable,
                graph_value
            );

            bgp_patterns.push(single_bgp);
        };
    }
    let serializer = Serializer {
        template: bgp_patterns.join("\n"),
        options:  None,
        format:   operator::formats::DataFormat::NQuads,
    };

    extended_plan.serialize(serializer)
}

fn add_rename_extend_op_from_quads(
    doc: &IndexedShExMLDocument,
    quads: &ShExMLQuads<'_>,
    sourced_plan: RcRefCellPlan<Processed>,
    variablized_terms: &IndexVariableTerm<'_>,
) -> Result<Plan<Processed>, PlanError> {
    let mut expression_extend_func_pairs: Vec<(String, Function)> = Vec::new();
    let expression_stmts_map = &doc.expression_stmts;
    let mut expr_ident_set = HashSet::new();
    let mut rename_pairs = HashMap::new();
    for (subj, _, obj, _) in quads {
        expr_ident_set.extend(subj.expression.extract_expr_idents());
        expr_ident_set.extend(obj.expression.extract_expr_idents());
    }

    trace!("Expression identifier set: {:#?}", expr_ident_set); 
    for expr_ident in expr_ident_set {
        if let Some(expression_stmt) = expression_stmts_map.get(expr_ident) {
            //Add string concatentation extend functions
            let concate_extend_pairs =
                extend::translate_concatenate_to_extend_pairs(
                    expr_ident,
                    &doc.iterators,
                    &expression_stmt.expr_enum,
                );
            expression_extend_func_pairs.extend(concate_extend_pairs);

            //Add rename pairs
            let rename_pairs_translated = rename::translate_rename_pairs_map(
                &doc.iterators,
                expression_stmt,
            );
            rename_pairs.extend(rename_pairs_translated);
        }
    }

    // Add concatenate extend functions as one extend operation

    let mut next_plan = sourced_plan.clone();
    trace!(
        "Extend function pairs for concatenation: {:#?}",
        expression_extend_func_pairs
    );
    next_plan = match !expression_extend_func_pairs.is_empty() {
        true => {
            debug!("Adding rename operator since it is not empty");
            let extend_pairs: HashMap<_, _> =
                expression_extend_func_pairs.into_iter().collect();

            let extend_op = operator::Operator::ExtendOp {
                config: Extend { extend_pairs },
            };
            Rc::new(
                (*next_plan)
                    .borrow_mut()
                    .apply(&extend_op, "Extend_Concatenate")?
                    .into(),
            )
        }
        false => next_plan,
    };

    trace!("Rename pairs: {:#?}", rename_pairs);
    next_plan = match !rename_pairs.is_empty() {
        true => {
            // Add rename operator to the extended plan
            debug!("Adding rename operator since it is not empty");
            let rename_op = operator::Operator::RenameOp {
                config: Rename { rename_pairs },
            };

            Rc::new(
                (*next_plan)
                    .borrow_mut()
                    .apply(&rename_op, "Rename_expression")?
                    .into(),
            )
        }
        false => next_plan,
    };

    // Add extend operator with the final values for triples serialization
    let sub_obj_map: HashMap<&Subject, Vec<(&Object, &ShapeIdent)>> =
        quads.iter().fold(HashMap::new(), |mut acc, quad| {
            let subj = quad.0;
            let pair = (quad.2, quad.3);
            if let Some(quads_mut) = acc.get_mut(subj) {
                quads_mut.push(pair);
            } else {
                acc.insert(subj, vec![pair]);
            }
            acc
        });

    let mut triples_extend_func_pairs: HashMap<String, Function> =
        HashMap::new();

    for (subj, obj_shape_pairs) in sub_obj_map.iter() {
        if let Some(subj_term_func) = extend::term::rdf_term_function(
            doc,
            Some(&subj.prefix),
            &subj.expression,
        ) {
            let subj_term_iri_func = Function::Iri {
                inner_function: subj_term_func.into(),
            };

            for (obj, _shape_ident) in obj_shape_pairs.iter() {
                let subj_variable =
                    variablized_terms.subject_variable_index.get(subj).unwrap();

                if triples_extend_func_pairs.get(subj_variable).is_none() {
                    triples_extend_func_pairs.insert(
                        subj_variable.to_string(),
                        subj_term_iri_func.clone(),
                    );
                }

                if let Some(obj_func) =
                    extend::term::obj_lang_datatype_function(doc, obj)
                {
                    let obj_variable = variablized_terms
                        .object_variable_index
                        .get(obj)
                        .unwrap();

                    triples_extend_func_pairs
                        .insert(obj_variable.to_string(), obj_func);
                }
            }
        }
    }

    //Let next_plan live a little longer
    let result = (*next_plan).borrow_mut().apply(
        &operator::Operator::ExtendOp {
            config: Extend {
                extend_pairs: triples_extend_func_pairs,
            },
        },
        "Extend_for_Serializer",
    );

    result
}
