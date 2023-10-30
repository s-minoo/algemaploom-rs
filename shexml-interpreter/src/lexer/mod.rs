mod tests;

use chumsky::chain::Chain;
use chumsky::prelude::*;
use chumsky::text::Character;
use chumsky::Parser;

use crate::token::ShExMLToken;

macro_rules! t {
    ($t:ty) => {
        impl Parser<char, $t, Error = Simple<char>> + Clone
    };
}

pub fn token(st: &'static str, token: ShExMLToken) -> t!(ShExMLToken) {
    just(st).padded().to(token)
}

pub fn within_angled_brackets() -> t!(String) {
    none_of("<>")
        .repeated()
        .at_least(1)
        .padded()
        .map(|c| c.into_iter().collect::<String>())
}

pub fn autoincrement() -> t!(Vec<ShExMLToken>) {
    todo()
}

pub fn matcher() -> t!(Vec<ShExMLToken>) {
    let mat_tag = token("MATCHER", ShExMLToken::Matcher);
    let mat_ident = ident().padded();
    let mats_value = none_of("<>,&")
        .repeated()
        .at_least(1)
        .padded()
        .map(|v_char: Vec<char>| v_char.into_iter().collect::<String>())
        .map(|str| {
            str.split(" AS ")
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
        })
        .map(|mut splitted_str| {
            let mut result = vec![];

            if splitted_str.len() > 1 {
                let ident = ShExMLToken::Ident(
                    splitted_str.pop().unwrap().trim().to_string(),
                );
                let value = ShExMLToken::Value(
                    splitted_str.pop().unwrap().trim().to_string(),
                );
                result.push(value);
                result.push(ShExMLToken::As);
                result.push(ident);
            } else {
                let value = splitted_str.pop().unwrap();
                result.push(ShExMLToken::Value(value.trim().to_string()));
            }
            result
        });

    mat_tag.chain(mat_ident).chain(
        token("<", ShExMLToken::AngleStart)
            .chain(
                mats_value
                    .then(
                        token(",", ShExMLToken::Comma)
                            .or(token("&", ShExMLToken::MatcherSplit))
                            .or_not(),
                    )
                    .map(|(mut tokens, opt_tok)| {
                        if let Some(delim_tok) = opt_tok {
                            tokens.push(delim_tok);
                        }

                        tokens
                    })
                    .repeated()
                    .at_least(1)
                    .flatten(),
            )
            .chain(token(">", ShExMLToken::AngleEnd)),
    )
}

pub fn expression() -> t!(Vec<ShExMLToken>) {
    let expressiont_tag = token("EXPRESSION", ShExMLToken::Expression);
    let exp_ident = ident().padded();

    let sub_ident = ident()
        .chain(just('.').to(ShExMLToken::Dot))
        .repeated()
        .at_least(1)
        .flatten()
        .chain(ident());

    let join_ident = token("JOIN", ShExMLToken::Join)
        .chain(sub_ident.clone())
        .padded();

    let union_ident = token("UNION", ShExMLToken::Union)
        .chain(sub_ident.clone())
        .padded();

    let join_union = sub_ident.clone().chain::<ShExMLToken, _, _>(
        join_ident.or(union_ident).repeated().at_least(1).flatten(),
    );

    let str_op_right = just('+')
        .padded()
        .ignored()
        .then(
            pn_char()
                .repeated()
                .at_least(1)
                .delimited_by(just('"'), just('"'))
                .map(|string_sep| {
                    ShExMLToken::StringSep(string_sep.into_iter().collect())
                })
                .then_ignore(just('+').padded()),
        )
        .then(sub_ident.clone().padded())
        .map(|((_, token), tokens)| {
            let mut result = vec![token];

            result.extend_from_slice(&tokens);
            result
        });

    let str_operation = sub_ident
        .clone()
        .chain(str_op_right.repeated().at_least(1).flatten());

    let exp_inner = token("<", ShExMLToken::AngleStart)
        .chain(
            join_union
                .or(str_operation)
                .repeated()
                .at_least(1)
                .flatten(),
        )
        .chain(token(">", ShExMLToken::AngleEnd));
    expressiont_tag.chain(exp_ident).chain(exp_inner)
}

pub fn iterator() -> t!(Vec<ShExMLToken>) {
    let header = iterator_header().padded();

    recursive(|iter| {
        header
            .chain(token("{", ShExMLToken::BrackStart))
            .chain::<ShExMLToken, _, _>(
                field().repeated().at_least(1).flatten(),
            )
            .chain::<ShExMLToken, _, _>(
                token("}", ShExMLToken::BrackEnd)
                    .map(|tok| vec![tok])
                    .or(iter),
            )
            .chain(token("}", ShExMLToken::BrackEnd).map(|tok| vec![tok]).or_not())
            .padded()
    })
}

pub fn field() -> t!(Vec<ShExMLToken>) {
    let field_tag = token("FIELD", ShExMLToken::Field);
    let push_field_tag = token("PUSHED_FIELD", ShExMLToken::PushField);
    let pop_field_tag = token("POPPED_FIELD", ShExMLToken::PopField);
    let field_name = ident().padded();
    let field_query = within_angled_brackets()
        .padded()
        .delimited_by(just("<"), just(">"))
        .map(ShExMLToken::FieldQuery);

    choice((field_tag, push_field_tag, pop_field_tag))
        .chain(field_name)
        .chain(field_query)
}

pub fn iterator_header() -> t!(Vec<ShExMLToken>) {
    let iterator_tag = token("ITERATOR", ShExMLToken::Iterator);
    let iterator_name = ident().padded();

    let iterator_type = protocol().padded().map(ShExMLToken::IteratorType);
    let iterator_query =
        within_angled_brackets().map(ShExMLToken::IteratorQuery);

    let iter_query_pair = token("<", ShExMLToken::AngleStart)
        .chain(iterator_type.chain(iterator_query))
        .chain(token(">", ShExMLToken::AngleEnd));

    iterator_tag.chain(iterator_name).chain(iter_query_pair)
}

pub fn source() -> t!(Vec<ShExMLToken>) {
    let source_tag = token("SOURCE", ShExMLToken::Source);
    let source_name = ident().padded();
    let source_iri = token("<", ShExMLToken::AngleStart)
        .chain(protocol_iri_ref().or(path().map(|st| ShExMLToken::URI(st))))
        .chain(token(">", ShExMLToken::AngleEnd));
    source_tag.chain(source_name).chain(source_iri)
}

pub fn ident() -> t!(ShExMLToken) {
    pn_char()
        .repeated()
        .at_least(1)
        .map(|v| ShExMLToken::Ident(v.into_iter().collect()))
}

pub fn prefix() -> t!(Vec<ShExMLToken>) {
    let prefix_tag = token("PREFIX", ShExMLToken::Prefix);
    let pname = pn_prefix()
        .or_not()
        .then_ignore(just(":"))
        .padded()
        .flatten()
        .map(|pname_vec| {
            if pname_vec.is_empty() {
                ShExMLToken::BasePrefix
            } else {
                let prefix = pname_vec.into_iter().collect();
                ShExMLToken::PrefixNS(prefix)
            }
        });

    prefix_tag.chain(pname).chain(
        token("<", ShExMLToken::AngleStart)
            .chain(protocol_iri_ref())
            .chain(token(">", ShExMLToken::AngleEnd)),
    )
}

pub fn protocol() -> t!(String) {
    filter(|c: &char| c.is_ascii_alphabetic())
        .repeated()
        .at_least(1)
        .chain(just(':'))
        .map(|c| c.into_iter().collect())
}

pub fn path() -> t!(String) {
    none_of("<>\"{}|^`\\[]")
        .repeated()
        .at_least(1)
        .map(|e| e.into_iter().collect())
}

pub fn protocol_iri_ref() -> t!(ShExMLToken) {
    let uri = protocol()
        .chain::<String, _, _>(just("//").map(|c| c.to_string()))
        .chain(path())
        .map(|z: Vec<String>| ShExMLToken::URI(z.into_iter().collect()));

    uri
}

fn pn_char_base() -> t!(char) {
    filter(|c: &char| c.is_alphabetic())
}

fn pn_char_u() -> t!(char) {
    pn_char_base().or(just('_'))
}
fn pn_char() -> t!(char) {
    pn_char_u()
        .or(just('-'))
        .or(filter(|c: &char| c.is_numeric()))
}
fn pn_prefix() -> t!(Vec<char>) {
    let ne = just('.')
        .repeated()
        .then(pn_char().repeated().at_least(1))
        .map(|(x, y)| {
            let mut o: Vec<char> = Vec::with_capacity(x.len() + y.len());
            x.append_to(&mut o);
            y.append_to(&mut o);
            o
        })
        .repeated()
        .flatten();

    pn_char_base().then(ne.or_not()).map(|(x, y)| {
        if let Some(y) = y {
            let mut o = Vec::with_capacity(y.len() + 1);
            o.push(x);
            o.extend(y);
            o
        } else {
            vec![x]
        }
    })
}
