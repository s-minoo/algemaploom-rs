pub mod r#type;
use chumsky::prelude::*;

use self::r#type::*;
use crate::token::*;

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
