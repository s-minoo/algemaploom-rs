use std::collections::HashMap;

use shexml_interpreter::{ExpressionStmt, FieldType, Iterator};

fn update_rename_map_iterator(
    prefix: &str,
    iterator: &Iterator,
    rename_pairs: &mut HashMap<String, String>,
) {
    let normal_fields = iterator
        .fields
        .iter()
        .filter(|field| field.field_type == FieldType::Normal);

    normal_fields.for_each(|field| {
        let from = field.ident.to_string();
        let to = format!("{}.{}", prefix, field.ident);
        rename_pairs.insert(from, to);
    });

    for nested_iter in iterator.nested_iterator.iter() {
        update_rename_map_iterator(
            &format!("{}.{}", prefix, nested_iter.ident),
            nested_iter,
            rename_pairs,
        );
    }
}

pub fn translate_rename_pairs_map(
    iterators_map: &HashMap<String, Iterator>,
    expr_stmt: &ExpressionStmt,
) -> HashMap<String, String> {
    let mut rename_pairs = HashMap::new();
    if let shexml_interpreter::ExpressionStmtEnum::Basic { reference } =
        &expr_stmt.expr_enum
    {
        let iter_ident = &reference.iterator_ident;
        let expr_ident = &expr_stmt.ident;

        if let Some(field) = &reference.field {
            let from = field.to_string();
            let to = format!("{}.{}", expr_ident, field);

            rename_pairs.insert(from, to);
        } else if let Some(iterator) = iterators_map.get(iter_ident) {
            update_rename_map_iterator(expr_ident, iterator, &mut rename_pairs);
        }
    }
    rename_pairs
}
