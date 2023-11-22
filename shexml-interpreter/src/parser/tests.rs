use std::collections::HashSet;

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
fn auto_inc_only_start_test() {
    let match_str = "
     AUTOINCREMENT myId <2>   
     ";

    let (tokens_opt, errors) = lexer::autoincrement()
        .padded()
        .then_ignore(end())
        .parse_recovery(match_str);

    println!("{:?}", tokens_opt);

    let (parsed_items, errors) =
        parser::auto_increments().parse_recovery(tokens_opt.unwrap());

    assert!(errors.len() == 0, "{:?}", errors);

    let expected_items = Some(vec![AutoIncrement {
        ident:  "myId".to_string(),
        start:  2,
        prefix: None,
        suffix: None,
        end:    None,
        step:   None,
    }]);

    assert_parse_expected(parsed_items, expected_items)
}

#[test]
fn auto_inc_start_test() {
    let match_str = "
     AUTOINCREMENT myId <\"my\" + 0 >   
     ";

    let (tokens_opt, errors) = lexer::autoincrement()
        .padded()
        .then_ignore(end())
        .parse_recovery(match_str);

    println!("{:?}", tokens_opt);

    let (parsed_items, errors) =
        parser::auto_increments().parse_recovery(tokens_opt.unwrap());

    assert!(errors.len() == 0, "{:?}", errors);

    let expected_items = Some(vec![AutoIncrement {
        ident:  "myId".to_string(),
        start:  0,
        prefix: Some("my".to_string()),
        suffix: None,
        end:    None,
        step:   None,
    }]);

    assert_parse_expected(parsed_items, expected_items)
}

#[test]
fn auto_inc_test() {
    let match_str = "
     AUTOINCREMENT myId <\"my\" + 0 to 10 by 2 + \"Id\">   
     ";

    let (tokens_opt, errors) = lexer::autoincrement()
        .padded()
        .then_ignore(end())
        .parse_recovery(match_str);

    println!("{:?}", tokens_opt);

    let (parsed_items, errors) =
        parser::auto_increments().parse_recovery(tokens_opt.unwrap());

    assert!(errors.len() == 0, "{:?}", errors);

    let expected_items = Some(vec![AutoIncrement {
        ident:  "myId".to_string(),
        start:  0,
        prefix: Some("my".to_string()),
        suffix: Some("Id".to_string()),
        end:    Some(10),
        step:   Some(2),
    }]);

    assert_parse_expected(parsed_items, expected_items)
}

#[test]
fn matcher_multiple_test() {
    let match_str = "
        MATCHER regions <Principality of Asturias, Principado de Asturias, Principáu d'Asturies, Asturies AS Asturias &
                Spain, España, Espagne AS Spain>
        ";

    let (tokens_opt, errors) = lexer::matcher()
        .padded()
        .then_ignore(end())
        .parse_recovery(match_str);
    let (parsed_items, errors) =
        parser::matchers().parse_recovery(tokens_opt.unwrap());

    assert!(errors.len() == 0, "{:?}", errors);

    let asturias_set: HashSet<_> = HashSet::from_iter(vec![
        "Principality of Asturias".to_string(),
        "Principado de Asturias".to_string(),
        "Principáu d'Asturies".to_string(),
        "Asturies".to_string(),
    ]);

    let spain_set: HashSet<_> = HashSet::from_iter(vec![
        "Spain".to_string(),
        "Espagne".to_string(),
        "España".to_string(),
    ]);

    let expected_items = Some(vec![Matcher {
        ident:      "regions".to_string(),
        rename_map: vec![
            ("Asturias".to_string(), asturias_set),
            ("Spain".to_string(), spain_set),
        ]
        .into_iter()
        .collect(),
    }]);

    assert_parse_expected(parsed_items, expected_items)
}

#[test]
fn matcher_single_test() {
    let match_str = "
        MATCHER ast <Principality of Asturias, Principado de Asturias, Principáu d'Asturies, Asturies AS Asturias>
        ";

    let (tokens_opt, errors) = lexer::matcher()
        .padded()
        .then_ignore(end())
        .parse_recovery(match_str);

    let (parsed_items, errors) =
        parser::matchers().parse_recovery(tokens_opt.unwrap());

    assert!(errors.len() == 0, "{:?}", errors);
    let values_set = HashSet::from_iter(vec![
        "Principality of Asturias".to_string(),
        "Principado de Asturias".to_string(),
        "Principáu d'Asturies".to_string(),
        "Asturies".to_string(),
    ]);

    let expected_items = Some(vec![Matcher {
        ident:      "ast".to_string(),
        rename_map: vec![("Asturias".to_string(), values_set)]
            .into_iter()
            .collect(),
    }]);

    assert_parse_expected(parsed_items, expected_items);
}

#[test]
fn expression_join_union_test() {
    let exp_str = "
        EXPRESSION exp <file.it1.name JOIN file.it2.name UNION file.it3.name>
        ";

    let (tokens_opt, errors) = lexer::expression()
        .padded()
        .then_ignore(end())
        .parse_recovery(exp_str);

    println!("{:?}", tokens_opt);
    let (parsed_items, errors) =
        parser::expressions().parse_recovery(tokens_opt.unwrap());

    assert!(errors.len() == 0, "{:?}", errors);

    let union_exp = Box::new(Expression::Union(
        Box::new(Expression::Basic {
            path: "file.it2.name".to_string(),
        }),
        Box::new(Expression::Basic {
            path: "file.it3.name".to_string(),
        }),
    ));
    let expression = Expression::Join(
        Box::new(Expression::Basic {
            path: "file.it1.name".to_string(),
        }),
        union_exp,
    );

    let expected_items = Some(vec![ExpressionStatement {
        ident: "exp".to_string(),
        expression,
    }]);

    assert_parse_expected(parsed_items, expected_items);
}

#[test]
fn expression_join_test() {
    let exp_str = "
        EXPRESSION exp <file.it1.name JOIN file.it2.name>
        ";

    let (tokens_opt, errors) = lexer::expression()
        .padded()
        .then_ignore(end())
        .parse_recovery(exp_str);

    println!("{:?}", tokens_opt);
    let (parsed_items, errors) =
        parser::expressions().parse_recovery(tokens_opt.unwrap());

    assert!(errors.len() == 0, "{:?}", errors);

    let expression = Expression::Join(
        Box::new(Expression::Basic {
            path: "file.it1.name".to_string(),
        }),
        Box::new(Expression::Basic {
            path: "file.it2.name".to_string(),
        }),
    );

    let expected_items = Some(vec![ExpressionStatement {
        ident: "exp".to_string(),
        expression,
    }]);

    assert_parse_expected(parsed_items, expected_items);
}

#[test]
fn expression_string_op_test() {
    let exp_str = "
        EXPRESSION exp <file.it1.id + \"-seper-\" +  file.it2.name>
        ";

    let (tokens_opt, errors) = lexer::expression()
        .padded()
        .then_ignore(end())
        .parse_recovery(exp_str);

    println!("{:?}", tokens_opt);
    let (parsed_items, errors) =
        parser::expressions().parse_recovery(tokens_opt.unwrap());

    assert!(errors.len() == 0, "{:?}", errors);

    let expression = Expression::ConcateString {
        left_path:      "file.it1.id".to_string(),
        concate_string: "-seper-".to_string(),
        right_path:     "file.it2.name".to_string(),
    };
    let expected_items = Some(vec![ExpressionStatement {
        ident: "exp".to_string(),
        expression,
    }]);

    assert_parse_expected(parsed_items, expected_items);
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

    let (tokens_opt, errors) = lexer::iterator()
        .then_ignore(end())
        .parse_recovery(iter_str);

    println!("{:#?}", tokens_opt);
    assert!(errors.len() == 0, "{:?}", errors);
    let parsed_items = parser::iterators().parse(tokens_opt.unwrap()).ok();

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
