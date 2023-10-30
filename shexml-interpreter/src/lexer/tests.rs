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
fn auto_inc_only_start_test() {
    let match_str = "
     AUTOINCREMENT myId <2>   
     ";

    let (tokens_opt, errors) = autoincrement()
        .padded()
        .then_ignore(end())
        .parse_recovery(match_str);

    println!("{:?}", tokens_opt);
    assert!(errors.len() == 0, "{:?}", errors);
    let expected = Some(vec![
        ShExMLToken::AutoIncrement,
        ShExMLToken::Ident("myId".to_string()),
        ShExMLToken::AngleStart,
        ShExMLToken::AutoIncStart(2),
        ShExMLToken::AngleEnd,
    ]);

    assert!(
        tokens_opt == expected,
        "Expected output is: {:#?}\nGenerated output was: {:#?}",
        tokens_opt,
        expected
    )
}

#[test]
fn auto_inc_end_test() {
    let match_str = "
     AUTOINCREMENT myId <\"my\" + 0 to 20>   
     ";

    let (tokens_opt, errors) = autoincrement()
        .padded()
        .then_ignore(end())
        .parse_recovery(match_str);

    println!("{:?}", tokens_opt);
    assert!(errors.len() == 0, "{:?}", errors);
    let expected = Some(vec![
        ShExMLToken::AutoIncrement,
        ShExMLToken::Ident("myId".to_string()),
        ShExMLToken::AngleStart,
        ShExMLToken::AutoIncPrefix("my".to_string()),
        ShExMLToken::AutoIncStart(0),
        ShExMLToken::AutoIncEnd(20),
        ShExMLToken::AngleEnd,
    ]);

    assert!(
        tokens_opt == expected,
        "Expected output is: {:#?}\nGenerated output was: {:#?}",
        tokens_opt,
        expected
    )
}

#[test]
fn auto_inc_start_test() {
    let match_str = "
     AUTOINCREMENT myId <\"my\" + 0 >   
     ";

    let (tokens_opt, errors) = autoincrement()
        .padded()
        .then_ignore(end())
        .parse_recovery(match_str);

    println!("{:?}", tokens_opt);
    assert!(errors.len() == 0, "{:?}", errors);
    let expected = Some(vec![
        ShExMLToken::AutoIncrement,
        ShExMLToken::Ident("myId".to_string()),
        ShExMLToken::AngleStart,
        ShExMLToken::AutoIncPrefix("my".to_string()),
        ShExMLToken::AutoIncStart(0),
        ShExMLToken::AngleEnd,
    ]);

    assert!(
        tokens_opt == expected,
        "Expected output is: {:#?}\nGenerated output was: {:#?}",
        tokens_opt,
        expected
    )
}

#[test]
fn auto_inc_complete_test() {
    let match_str = "
     AUTOINCREMENT myId <\"my\" + 0 to 10 by 2 + \"Id\">   
     ";

    let (tokens_opt, errors) = autoincrement()
        .padded()
        .then_ignore(end())
        .parse_recovery(match_str);

    println!("{:?}", tokens_opt);
    assert!(errors.len() == 0, "{:?}", errors);
    let expected = Some(vec![
        ShExMLToken::AutoIncrement,
        ShExMLToken::Ident("myId".to_string()),
        ShExMLToken::AngleStart,
        ShExMLToken::AutoIncPrefix("my".to_string()),
        ShExMLToken::AutoIncStart(0),
        ShExMLToken::AutoIncEnd(10),
        ShExMLToken::AutoIncStep(2),
        ShExMLToken::AutoIncSuffix("Id".to_string()),
        ShExMLToken::AngleEnd,
    ]);

    assert!(
        tokens_opt == expected,
        "Expected output is: {:#?}\nGenerated output was: {:#?}",
        tokens_opt,
        expected
    )
}

#[test]
fn multiple_matching_matcher_test() {
    let match_str = "
        MATCHER regions <Principality of Asturias, Principado de Asturias, Principáu d'Asturies, Asturies AS Asturias &
                Spain, España, Espagne AS Spain>
        ";

    let (tokens_opt, errors) = matcher()
        .padded()
        .then_ignore(end())
        .parse_recovery(match_str);

    println!("{:?}", tokens_opt);
    assert!(errors.len() == 0, "{:?}", errors);
}

#[test]
fn single_matcher_test() {
    let match_str = "
        MATCHER ast <Principality of Asturias, Principado de Asturias, Principáu d'Asturies, Asturies AS Asturias>
        ";

    let (tokens_opt, errors) = matcher()
        .padded()
        .then_ignore(end())
        .parse_recovery(match_str);

    println!("{:?}", tokens_opt);
    assert!(errors.len() == 0, "{:?}", errors);
}

#[test]
fn string_op_expression_test() {
    let exp_str = "
        EXPRESSION exp <file.it1.id + \"-seper-\" +  file.it2.name>
        ";

    let (tokens_opt, errors) =
        expression().padded().then(end()).parse_recovery(exp_str);

    assert!(errors.len() == 0, "{:?}", errors);
    println!("{:?}", tokens_opt);
}

#[test]
fn join_union_expression_test() {
    let exp_str = "
        EXPRESSION exp <file.it1.id UNION file.it2.name UNION file.it1.name>
        ";

    let (tokens_opt, errors) =
        expression().padded().then(end()).parse_recovery(exp_str);

    assert!(errors.len() == 0, "{:?}", errors);
    println!("{:?}", tokens_opt);
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
        ITERATOR nestedIterator <jsonpath: nestedElements[*]> {
            POPPED_FIELD field2 <field1>
            FIELD field3 <field3>
        }
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
        ShExMLToken::AngleStart,
        ShExMLToken::IteratorType("xpath:".to_string()),
        ShExMLToken::IteratorQuery("/path/to/entity".to_string()),
        ShExMLToken::AngleEnd,
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
        ShExMLToken::AngleStart,
        ShExMLToken::URI("file.json".to_string()),
        ShExMLToken::AngleEnd,
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
        ShExMLToken::AngleStart,
        ShExMLToken::URI("https://example.com/file.xml".to_string()),
        ShExMLToken::AngleEnd,
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
        ShExMLToken::AngleStart,
        ShExMLToken::URI("https://base.com/".to_string()),
        ShExMLToken::AngleEnd,
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
        ShExMLToken::AngleStart,
        ShExMLToken::URI("https://example.com/".to_string()),
        ShExMLToken::AngleEnd,
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
