mod tests;
pub mod r#type;
use std::collections::HashMap;

use chumsky::prelude::*;

use self::r#type::*;
use crate::token::*;

macro_rules! shex_just {
    ($exp:expr) => {
        just::<ShExMLToken, _, Simple<ShExMLToken>>($exp)
    };
}

macro_rules! t {
    ($t:ty) => {
        impl Parser<ShExMLToken, $t, Error = Simple<ShExMLToken>> + Clone
    };
}

macro_rules! extract_string {
    ($t:ident) => {
        select! {
         ShExMLToken::$t(string) => string
        }
    };
}
fn token_string<T: AsRef<str> + Clone>(
    tok: ShExMLToken,
    target: T,
) -> t!(String) {
    just(tok).map(move |_| target.as_ref().to_string())
}

fn matchers() -> t!(Matcher) {
    let field_values = extract_string!(Value)
        .chain::<String, _, _>(
            shex_just!(ShExMLToken::Comma)
                .ignore_then(extract_string!(Value))
                .repeated(),
        )
        .then_ignore(just(ShExMLToken::As))
        .then(extract_string!(Ident))
        .map(|(values, key)| (key, values));

    just(ShExMLToken::Matcher)
        .ignore_then(extract_string!(Ident))
        .then::<Vec<(String, Vec<String>)>, _>(
            just(ShExMLToken::AngleStart)
                .ignore_then(field_values.clone())
                .chain(
                    just(ShExMLToken::MatcherSplit)
                        .ignore_then(field_values)
                        .repeated(),
                )
                .then_ignore(just(ShExMLToken::AngleEnd)),
        )
        .map(|(ident, key_values_vec)| {
            let mut rename_map = HashMap::new();

            for (key, values) in key_values_vec {
                rename_map.insert(key, values.into_iter().collect());
            }

            Matcher { ident, rename_map }
        })
}

fn exp_ident() -> t!(String) {
    extract_string!(Ident)
        .chain(
            just(ShExMLToken::Dot)
                .ignore_then(extract_string!(Ident))
                .repeated()
                .at_least(1),
        )
        .map(|strings: Vec<String>| strings.join("."))
}

fn expressions() -> t!(Vec<ExpressionStatement>) {
    just::<ShExMLToken, _, Simple<ShExMLToken>>(ShExMLToken::Expression)
        .ignore_then(extract_string!(Ident))
        .then(exp_join_union().or(exp_string_op()))
        .map(|(name, expression)| ExpressionStatement { name, expression })
        .repeated()
        .at_least(1)
}

fn exp_join_union() -> t!(Expression) {
    let basic_expression = exp_ident().map(|path| Expression::Basic { path });
    basic_expression
        .clone()
        .then(
            just(ShExMLToken::Union)
                .to(Expression::Union as fn(_, _) -> _)
                .or(just(ShExMLToken::Join)
                    .to(Expression::Join as fn(_, _) -> _))
                .then(basic_expression.clone())
                .repeated(),
        )
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
}

fn exp_string_op() -> t!(Expression) {
    exp_ident()
        .then(extract_string!(StringSep))
        .then(exp_ident())
        .map(|((left_path, concate_string), right_path)| {
            Expression::ConcateString {
                left_path,
                concate_string,
                right_path,
            }
        })
}

fn sources() -> t!(Vec<Source>) {
    just(ShExMLToken::Source)
        .ignore_then(extract_string!(Ident))
        .then(extract_string!(URI).delimited_by(
            just(ShExMLToken::AngleStart),
            just(ShExMLToken::AngleEnd),
        ))
        .map(|(ident, uri)| Source { name: ident, uri })
        .repeated()
        .at_least(1)
}

fn iterators() -> t!(Vec<Box<Iterator>>) {
    let normal_fields = fields(ShExMLToken::Field);
    let popped_fields = fields(ShExMLToken::PopField);
    let pushed_fields = fields(ShExMLToken::PushField);

    let fields = normal_fields
        .or(popped_fields)
        .or(pushed_fields)
        .repeated()
        .at_least(1)
        .flatten();

    recursive(|iter| {
        just::<ShExMLToken, _, Simple<ShExMLToken>>(ShExMLToken::Iterator)
            .ignore_then(extract_string!(Ident))
            .then(
                extract_string!(IteratorType)
                    .or_not()
                    .then(extract_string!(IteratorQuery))
                    .map(|(opt_type, query)| {
                        if let Some(iter_type) = opt_type {
                            (iter_type, query)
                        } else {
                            ("".to_string(), query)
                        }
                    })
                    .delimited_by(
                        just(ShExMLToken::AngleStart),
                        just(ShExMLToken::AngleEnd),
                    ),
            )
            .then_ignore(just(ShExMLToken::BrackStart))
            .then(fields)
            .map(|((ident, (iter_type, query)), fields)| {
                (ident, iter_type, query, fields)
            })
            .then(iter.or_not())
            .then_ignore(just(ShExMLToken::BrackEnd))
            .map(|((ident, iter_type, query, fields), iterator_opt)| {
                Box::new(Iterator {
                    ident,
                    query,
                    iter_type,
                    fields,
                    nested_iterator: iterator_opt,
                })
            })
    })
    .repeated()
    .at_least(1)
}

fn fields(field_type_token: ShExMLToken) -> t!(Vec<Field>) {
    let field_type = match field_type_token {
        ShExMLToken::PushField => FieldType::Push,
        ShExMLToken::Field => FieldType::Normal,
        ShExMLToken::PopField => FieldType::Pop,
        _ => FieldType::Normal,
    };

    just(field_type_token)
        .ignore_then(extract_string!(Ident))
        .then(extract_string!(FieldQuery))
        .map(move |(name, query)| {
            Field {
                name,
                query,
                field_type,
            }
        })
        .repeated()
}

fn prefixes() -> t!(Vec<PrefixNameSpace>) {
    let string_val_parser = select! {
        ShExMLToken::PrefixNS(ns) => ns,
        ShExMLToken::BasePrefix => "".to_string(),
        ShExMLToken::URI(uri) => uri,

    };

    just(ShExMLToken::Prefix)
        .ignore_then(string_val_parser)
        .then(string_val_parser.delimited_by(
            just(ShExMLToken::AngleStart),
            just(ShExMLToken::AngleEnd),
        ))
        .map(|(prefix, local)| PrefixNameSpace { prefix, local })
        .repeated()
        .at_least(1)
}
