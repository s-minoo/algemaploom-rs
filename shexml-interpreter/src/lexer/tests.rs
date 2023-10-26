#[cfg(test)]
use super::*;
#[test]
fn protocol_test() {
    let proc1 = "jdbc:".to_string();
    let proc2 = "http:".to_string();
    let protocol_str = proc1.clone() + &proc2;

    let (tokens_opt, errors) = protocol()
        .repeated()
        .at_least(1)
        .parse_recovery(protocol_str);

    assert!(errors.len() == 0, "{:?}", errors);
    let expected_tokens = Some(vec![proc1, proc2]);
    assert!(
        tokens_opt == expected_tokens,
        "{:?} is the parsed tokens
            {:?} is the expected tokens
            ",
        tokens_opt,
        expected_tokens
    );
}

#[test]
fn iterator_test() {
    let iter_str = "
ITERATOR example <xpath: /path/to/entity> {
    FIELD field1 <@attribute>
    FIELD field2 <field2>
    FIELD field3 <path/to/field3>
}";

    let (tokens_opt, errors) = iterator().then(end()).parse_recovery(iter_str);

    assert!(errors.len() == 0, "{:?}", errors);
    println!("{:?}", tokens_opt);
    
}


#[test]
fn iterator_nest_test() {
    let iter_str = "
    ITERATOR example <jsonpath: $> {
    PUSHED_FIELD field1 <id>
    ITERATOR nestedIterator <jsonpath: nestedElements[*]> {
        POPPED_FIELD field2 <field1>
        FIELD field3 <field3>
    }
}";

    let (tokens_opt, errors) = iterator().then(end()).parse_recovery(iter_str);

    assert!(errors.len() == 0, "{:?}", errors);
    println!("{:?}", tokens_opt);
}

#[test]
fn iterator_header_test() {
    let iter_str = "ITERATOR example <xpath: /path/to/entity>";
    let (tokens_opt, errors) = iterator_header()
        .padded()
        .then_ignore(end())
        .parse_recovery(iter_str);
    assert!(errors.len() == 0, "{:?}", errors);
    let expected_tokens = Some(vec![
        ShExMLToken::Iterator,
        ShExMLToken::Ident("example".to_string()),
        ShExMLToken::IteratorType("xpath:".to_string()),
        ShExMLToken::IteratorQuery("/path/to/entity".to_string()),
    ]);
    assert!(
        tokens_opt == expected_tokens,
        "{:?} is the parsed tokens
            {:?} is the expected tokens
            ",
        tokens_opt,
        expected_tokens
    );
}

#[test]
fn source_local_path_test() {
    let source_str = "SOURCE json_file <file.json>";
    let (tokens_opt, errors) = source().parse_recovery(source_str);
    assert!(errors.len() == 0, "{:?}", errors);
    let expected_tokens = Some(vec![
        ShExMLToken::Source,
        ShExMLToken::Ident("json_file".to_string()),
        ShExMLToken::URI("file.json".to_string()),
    ]);
    assert!(
        tokens_opt == expected_tokens,
        "{:?} is the parsed tokens
            {:?} is the expected tokens
            ",
        tokens_opt,
        expected_tokens
    );
}

#[test]
fn source_test() {
    let source_str = "SOURCE xml_file <https://example.com/file.xml>";
    let (tokens_opt, errors) = source().parse_recovery(source_str);
    assert!(errors.len() == 0, "{:?}", errors);
    let expected_tokens = Some(vec![
        ShExMLToken::Source,
        ShExMLToken::Ident("xml_file".to_string()),
        ShExMLToken::URI("https://example.com/file.xml".to_string()),
    ]);
    assert!(
        tokens_opt == expected_tokens,
        "{:?} is the parsed tokens
            {:?} is the expected tokens
            ",
        tokens_opt,
        expected_tokens
    );
}

#[test]
fn empty_prefix_name_test() {
    let base_prefix = "PREFIX : <https://base.com/>";

    let (tokens_opt, errors) = prefix().parse_recovery(base_prefix);

    assert!(errors.len() == 0, "{:?}", errors);
    let expected_tokens = Some(vec![
        ShExMLToken::Prefix,
        ShExMLToken::BasePrefix,
        ShExMLToken::URI("https://base.com/".to_string()),
    ]);
    assert!(
        tokens_opt == expected_tokens,
        "{:?} is the parsed tokens
            {:?} is the expected tokens
            ",
        tokens_opt,
        expected_tokens
    );
}

#[test]
fn prefix_test() {
    let prefix_1 = "PREFIX ex: <https://example.com/>";

    let (tokens_opt, errors) = prefix().parse_recovery(prefix_1);

    assert!(errors.len() == 0, "{:?}", errors);
    let expected_tokens = Some(vec![
        ShExMLToken::Prefix,
        ShExMLToken::PrefixNS("ex".to_string()),
        ShExMLToken::URI("https://example.com/".to_string()),
    ]);
    assert!(
        tokens_opt == expected_tokens,
        "{:?} is the parsed tokens
            {:?} is the expected tokens
            ",
        tokens_opt,
        expected_tokens
    );
}
