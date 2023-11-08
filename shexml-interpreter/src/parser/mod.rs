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

macro_rules! unfold_token_value {
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

fn shapes() -> t!(Vec<Shape>) {
    todo()
}

fn functions() -> t!(Vec<Function>) {
    shex_just!(ShExMLToken::Function)
        .ignore_then(unfold_token_value!(Ident))
        .then(
            unfold_token_value!(FunctionLang)
                .then(unfold_token_value!(URI))
                .delimited_by(
                    just(ShExMLToken::AngleStart),
                    just(ShExMLToken::AngleEnd),
                ),
        )
        .map(|(ident, (lang_type, uri))| {
            Function {
                ident,
                lang_type,
                uri,
            }
        })
        .repeated()
}

fn auto_increments() -> t!(Vec<AutoIncrement>) {
    let auto_inc_ident_exp = unfold_token_value!(Ident)
        .then(
            unfold_token_value!(AutoIncPrefix)
                .or_not()
                .then(unfold_token_value!(AutoIncStart))
                .then(unfold_token_value!(AutoIncStep).or_not())
                .then(unfold_token_value!(AutoIncEnd).or_not())
                .then(unfold_token_value!(AutoIncSuffix).or_not())
                .delimited_by(
                    just(ShExMLToken::AngleStart),
                    just(ShExMLToken::AngleEnd),
                ),
        )
        .map(|(ident, ((((prefix, start), step), end), suffix))| {
            AutoIncrement {
                ident,
                start,
                prefix,
                suffix,
                end,
                step,
            }
        });

    just(ShExMLToken::AutoIncrement)
        .ignore_then(auto_inc_ident_exp)
        .repeated()
}

fn matchers() -> t!(Vec<Matcher>) {
    let field_values = unfold_token_value!(Value)
        .chain::<String, _, _>(
            shex_just!(ShExMLToken::Comma)
                .ignore_then(unfold_token_value!(Value))
                .repeated(),
        )
        .then_ignore(just(ShExMLToken::As))
        .then(unfold_token_value!(Ident))
        .map(|(values, key)| (key, values));

    just(ShExMLToken::Matcher)
        .ignore_then(unfold_token_value!(Ident))
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
        .repeated()
}

fn exp_ident() -> t!(String) {
    unfold_token_value!(Ident)
        .chain(
            just(ShExMLToken::Dot)
                .ignore_then(unfold_token_value!(Ident))
                .repeated()
                .at_least(1),
        )
        .map(|strings: Vec<String>| strings.join("."))
}

fn expressions() -> t!(Vec<ExpressionStatement>) {
    just::<ShExMLToken, _, Simple<ShExMLToken>>(ShExMLToken::Expression)
        .ignore_then(unfold_token_value!(Ident))
        .then(exp_join_union().or(exp_string_op()))
        .map(|(name, expression)| ExpressionStatement { ident: name, expression })
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
        .then(unfold_token_value!(StringSep))
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
        .ignore_then(unfold_token_value!(Ident))
        .then(unfold_token_value!(URI).delimited_by(
            just(ShExMLToken::AngleStart),
            just(ShExMLToken::AngleEnd),
        ))
        .map(|(ident, uri)| Source { ident, uri })
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
            .ignore_then(unfold_token_value!(Ident))
            .then(
                unfold_token_value!(IteratorType)
                    .or_not()
                    .then(unfold_token_value!(IteratorQuery))
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
        .ignore_then(unfold_token_value!(Ident))
        .then(unfold_token_value!(FieldQuery))
        .map(move |(name, query)| {
            Field {
                ident: name,
                query,
                field_type,
            }
        })
        .repeated()
}

fn prefixes() -> t!(Vec<Prefix>) {
    let prefix_ns = select! {
        ShExMLToken::PrefixNS(ns) => ns,
        ShExMLToken::BasePrefix => "".to_string(),

    };

    just(ShExMLToken::Prefix)
        .ignore_then(prefix_ns)
        .then(unfold_token_value!(URI).delimited_by(
            just(ShExMLToken::AngleStart),
            just(ShExMLToken::AngleEnd),
        ))
        .map(|(prefix, uri)| Prefix { prefix, uri })
        .repeated()
        .at_least(1)
}
