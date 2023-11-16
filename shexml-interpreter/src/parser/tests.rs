#[cfg(test)]
use super::*;
use crate::{lexer, parser};

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
    assert!(
        parsed_items == expected_items,
        "{:?} is the parsed items
            {:?} is the expected items
            ",
        parsed_items,
        expected_items
    );
}
