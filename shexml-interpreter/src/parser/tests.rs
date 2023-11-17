#[cfg(test)]
use super::*;
use crate::{lexer, parser};

fn assert_parse_expected<T: std::fmt::Debug + PartialEq + Eq>(
    parsed_items: Option<Vec<T>>,
    expected_items: Option<Vec<T>>,
) {
    assert!(
        parsed_items == expected_items,
        "{:?} is the parsed items
            {:?} is the expected items
            ",
        parsed_items,
        expected_items
    );
}

#[test]
fn iterator_nested_test() {
    let iter_str = "
    ITERATOR example <jsonpath: $> {
    PUSHED_FIELD field1 <id>
    ITERATOR nestedIterator <jsonpath: nestedElements[*]> {
        POPPED_FIELD field2 <field1>
        FIELD field3 <field3>
        ITERATOR nestedIterator <jsonpath: nestedElements[*]> {
            POPPED_FIELD field2 <field1>
            FIELD field3 <field3>
        }
    }
}";

    let (tokens_opt, errors) =
        lexer::iterator().then(end()).parse_recovery(iter_str);

    let inner_fields = vec![
        Field {
            field_type: FieldType::Pop,
            ident:      "field2".to_string(),
            query:      "field1".to_string(),
        },
        Field {
            field_type: FieldType::Normal,
            ident:      "field3".to_string(),
            query:      "field3".to_string(),
        },
    ];

    let innermost_iter = Iterator {
        ident:           "nestedIterator".to_string(),
        query:           "nestedElements[*]".to_string(),
        iter_type:       "jsonpath:".to_string(),
        fields:          inner_fields,
        nested_iterator: None,
    };

    let inner_iter = Iterator {
        nested_iterator: Some(Box::new(innermost_iter.clone())),
        ..innermost_iter
    };

    let fields = vec![Field {
        field_type: FieldType::Push,
        ident:      "field1".to_string(),
        query:      "id".to_string(),
    }];

    let expected_items = Some(vec![Box::new(Iterator {
        ident: "example".to_string(),
        query: "$".to_string(),
        iter_type: "jsonpath:".to_string(),
        fields,
        nested_iterator: Some(Box::new(inner_iter)),
    })]);

    let (parsed_items, errors) =
        parser::iterators().parse_recovery(tokens_opt.unwrap().0);

    assert!(errors.len() == 0, "{:?}", errors);
    assert_parse_expected(parsed_items, expected_items)
}

#[test]
fn iterator_test() {
    let iter_str = "
ITERATOR example <xpath: /path/to/entity> {
    FIELD field1 <@attribute>
    FIELD field2 <field2>
    FIELD field3 <path/to/field3>
}";

    let (tokens_opt, errors) =
        lexer::iterator().then(end()).parse_recovery(iter_str);

    println!("{:#?}", tokens_opt);
    assert!(errors.len() == 0, "{:?}", errors);
    let parsed_items = parser::iterators().parse(tokens_opt.unwrap().0).ok();

    let fields = vec![
        Field {
            field_type: FieldType::Normal,
            ident:      "field1".to_string(),
            query:      "@attribute".to_string(),
        },
        Field {
            field_type: FieldType::Normal,
            ident:      "field2".to_string(),
            query:      "field2".to_string(),
        },
        Field {
            field_type: FieldType::Normal,
            ident:      "field3".to_string(),
            query:      "path/to/field3".to_string(),
        },
    ];

    let expected_items = Some(vec![Box::new(Iterator {
        ident: "example".to_string(),
        query: "/path/to/entity".to_string(),
        iter_type: "xpath:".to_string(),
        fields,
        nested_iterator: None,
    })]);
    assert_parse_expected(parsed_items, expected_items)
}

#[test]
fn prefix_multiple_test() {
    let prefix_1 = r#"PREFIX ex: <https://example.com/>
    PREFIX ex23: <https://example23.com/>

        "#;

    let (tokens_opt, _) = lexer::prefix().parse_recovery(prefix_1);
    println!("{:?}", tokens_opt);
    let (parsed_items, error) =
        parser::prefixes().parse_recovery(tokens_opt.unwrap());

    assert!(error.len() == 0, "{:#?}", error);
    let expected_items = Some(vec![
        Prefix {
            prefix: "ex".to_string(),
            uri:    "https://example.com/".to_string(),
        },
        Prefix {
            prefix: "ex23".to_string(),
            uri:    "https://example23.com/".to_string(),
        },
    ]);
    assert_parse_expected(parsed_items, expected_items);
}

#[test]
fn prefix_test() {
    let prefix_1 = "PREFIX ex: <https://example.com/>";

    let (tokens_opt, _) = lexer::prefix().parse_recovery(prefix_1);
    println!("{:?}", tokens_opt);
    let (parsed_items, error) =
        parser::prefixes().parse_recovery(tokens_opt.unwrap());

    assert!(error.len() == 0, "{:#?}", error);
    let expected_items = Some(vec![Prefix {
        prefix: "ex".to_string(),
        uri:    "https://example.com/".to_string(),
    }]);

    assert_parse_expected(parsed_items, expected_items);
}

#[test]
fn source_multiple_test() {
    let source_str = r#"SOURCE xml_file <https://example.com/file.xml>
    SOURCE json_file <local/file.json>

        "#;
    let (tokens_opt, _) = lexer::source().parse_recovery(source_str);
    println!("{:?}", tokens_opt);
    let (parsed_items, errors) =
        parser::sources().parse_recovery(tokens_opt.clone().unwrap());

    assert!(errors.len() == 0, "{:?}", errors);
    let expected_items = Some(vec![
        Source {
            ident: "xml_file".to_string(),
            uri:   "https://example.com/file.xml".to_string(),
        },
        Source {
            ident: "json_file".to_string(),
            uri:   "local/file.json".to_string(),
        },
    ]);
    assert_parse_expected(parsed_items, expected_items)
}

#[test]
fn source_test() {
    let source_str = "SOURCE xml_file <https://example.com/file.xml>";
    let (tokens_opt, _) = lexer::source().parse_recovery(source_str);
    let (parsed_items, errors) =
        parser::sources().parse_recovery(tokens_opt.clone().unwrap());

    assert!(errors.len() == 0, "{:?}", errors);
    let expected_items = Some(vec![Source {
        ident: "xml_file".to_string(),
        uri:   "https://example.com/file.xml".to_string(),
    }]);
    assert_parse_expected(parsed_items, expected_items)
}
