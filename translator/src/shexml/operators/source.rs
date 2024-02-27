use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use operator::formats::ReferenceFormulation;
use operator::{IOType, Source};
use shexml_interpreter::{ExpressionStmtEnum, Iterator, ShExMLDocument};

use crate::OperatorTranslator;

#[derive(Debug, Clone)]
pub struct ShExMLSourceTranslator<'a> {
    pub document: &'a ShExMLDocument,
}

pub type SourceExprIdentVecPair = (Source, Vec<String>);

impl<'a> OperatorTranslator<HashMap<String, SourceExprIdentVecPair>>
    for ShExMLSourceTranslator<'a>
{
    fn translate(&self) -> HashMap<String, SourceExprIdentVecPair> {
        let ident_config_iotype_map: HashMap<_, _> = self
            .document
            .sources
            .iter()
            .map(|source| {
                let mut config = HashMap::new();
                config.insert("url".to_string(), source.uri.clone());
                let source_type = match source.source_type {
                    shexml_interpreter::SourceType::File => IOType::File,
                    _ => unimplemented!(),
                };
                (source.ident.as_str(), (config, source_type))
            })
            .collect();

        let ident_iterators_map: HashMap<_, _> = self
            .document
            .iterators
            .iter()
            .map(|iter| (iter.ident.clone(), iter))
            .collect();

        let source_iter_pairs: Vec<((&str, &str), &str)> = self
            .document
            .expression_stmts
            .iter()
            .flat_map(|expr_stmt| {
                extract_source_iter_pairs(&expr_stmt.expr_enum)
                    .into_iter()
                    .map(|pair| (pair, expr_stmt.ident.as_str()))
            })
            .collect();

        let mut source_expr_idents_map = HashMap::new();
        for ((source_ident, iter_ident), expr_ident) in source_iter_pairs {
            let key = format!("{}{}", source_ident, expr_ident);
            if let Some(source_exprs_pair) =
                source_expr_idents_map.get_mut(&key)
            {
                let (_, exprs): &mut (Source, Vec<String>) = source_exprs_pair;

                exprs.push(expr_ident.to_string());
            } else {
                let config_iotype_pair =
                    ident_config_iotype_map.get(source_ident).unwrap();

                let iter = ident_iterators_map.get(iter_ident).unwrap();
                let source = Source {
                    config:        config_iotype_pair.0.clone(),
                    source_type:   config_iotype_pair.1.clone(),
                    root_iterator: translate_to_operator_iterator(iter),
                };

                let value = (source, vec![expr_ident.to_string()]);

                source_expr_idents_map.insert(key, value);
            }
        }

        source_expr_idents_map
    }
}

fn extract_source_iter_pairs(
    expr_enum: &ExpressionStmtEnum,
) -> HashSet<(&str, &str)> {
    let mut result = HashSet::new();
    match expr_enum {
        ExpressionStmtEnum::Union(left_box, right_box)
        | ExpressionStmtEnum::Join(left_box, right_box) => {
            let left_pairs = extract_source_iter_pairs(left_box);
            let right_pairs = extract_source_iter_pairs(right_box);

            result.extend(left_pairs);
            result.extend(right_pairs);
        }
        ExpressionStmtEnum::ConcatenateString {
            left_reference,
            right_reference,
            ..
        } => {
            result.insert((
                &left_reference.source_ident,
                &left_reference.iterator_ident,
            ));
            result.insert((
                &right_reference.source_ident,
                &right_reference.iterator_ident,
            ));
        }
        ExpressionStmtEnum::Basic { reference } => {
            result.insert((&reference.source_ident, &reference.iterator_ident));
        }
    };

    result
}

fn translate_to_operator_iterator(
    shexml_iter: &Iterator,
) -> operator::Iterator {
    let reference_formulation = translate_to_reference_formulation(
        shexml_iter.iter_type.as_ref().unwrap(),
    );

    let reference = shexml_iter.query.to_string();
    let fields =
        translate_to_operator_fields(shexml_iter, &reference_formulation);

    operator::Iterator {
        reference: Some(reference),
        reference_formulation,
        fields,
    }
}

fn translate_to_reference_formulation(
    shex_iter_type: &shexml_interpreter::IteratorType,
) -> ReferenceFormulation {
    match shex_iter_type {
        shexml_interpreter::IteratorType::JSONPath => {
            ReferenceFormulation::JSONPath
        }
        shexml_interpreter::IteratorType::XPath => {
            ReferenceFormulation::XMLPath
        }
        shexml_interpreter::IteratorType::CSVRows => {
            ReferenceFormulation::CSVRows
        }
        shexml_interpreter::IteratorType::SQL => ReferenceFormulation::SQLQuery,
        shexml_interpreter::IteratorType::SPARQL => {
            ReferenceFormulation::SPARQL
        }
    }
}

fn translate_to_operator_fields(
    parent_shex_iter: &shexml_interpreter::Iterator,
    ref_formulation: &ReferenceFormulation,
) -> Vec<operator::Field> {
    let mut result = Vec::new();
    let flat_fields: Vec<operator::Field> = parent_shex_iter
        .fields
        .iter()
        .filter_map(|f| translate_to_flat_fields(f, ref_formulation))
        .collect();
    result.extend(flat_fields);

    let nested_iterator_fields: Vec<operator::Field> = parent_shex_iter
        .nested_iterator
        .iter()
        .map(|nested_iter| {
            operator::Field {
                alias:                 nested_iter.ident.clone(),
                reference:             nested_iter.query.clone(),
                reference_formulation: ref_formulation.clone(),
                inner_fields:          translate_to_operator_fields(
                    nested_iter,
                    ref_formulation,
                ),
            }
        })
        .collect();

    result.extend(nested_iterator_fields);
    result
}

fn translate_to_flat_fields(
    shex_field: &shexml_interpreter::Field,
    ref_formulation: &ReferenceFormulation,
) -> Option<operator::Field> {
    match shex_field.field_type {
        shexml_interpreter::FieldType::Push
        | shexml_interpreter::FieldType::Normal => {
            Some(operator::Field {
                alias:                 shex_field.ident.clone(),
                reference:             shex_field.query.clone(),
                reference_formulation: ref_formulation.clone(),
                inner_fields:          vec![],
            })
        }
        shexml_interpreter::FieldType::Pop => None,
    }
}
