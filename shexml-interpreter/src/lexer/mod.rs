mod tests;

use chumsky::chain::Chain;
use chumsky::prelude::*;
use chumsky::Parser;

use crate::token::{self, ShExMLToken};

macro_rules! t {
    ($t:ty) => {
        impl Parser<char, $t, Error = Simple<char>> + Clone
    };
}

macro_rules! shape_node {
    ($t:ident) => {
        pn_prefix()
            .or_not()
            .then_ignore(just(':'))
            .then(pn_char().repeated().at_least(1))
            .map(|(prefix_opt, local)| {
                let mut prefix: String = "".to_string();
                let local = local.into_iter().collect();
                if let Some(found_prefix) = prefix_opt {
                    prefix = found_prefix.into_iter().collect();
                }
                ShExMLToken::$t { prefix, local }
            })
    };
}

fn token(st: &'static str, token: ShExMLToken) -> t!(ShExMLToken) {
    just(st).padded().to(token)
}

fn within_angled_brackets() -> t!(String) {
    none_of("<>")
        .repeated()
        .at_least(1)
        .padded()
        .map(|c| c.into_iter().collect::<String>())
}

pub fn shapes() -> t!(Vec<ShExMLToken>) {
    let shape_node = shape_node!(ShapeNode);
    let predicate = shape_node!(ShapeTerm);
    let graph_node = shape_node!(ShapeNode);

    let pred_object = predicate
        .padded()
        .chain(shape_object())
        .padded()
        .chain(token(";", ShExMLToken::PredicateSplit));

    let subject =
        prefix_namespace().chain::<ShExMLToken, _, _>(shape_node_expression());

    let single_shape = shape_node
        .padded()
        .chain(subject)
        .padded()
        .chain(token("{", ShExMLToken::CurlStart))
        .padded()
        .chain::<ShExMLToken, _, _>(
            pred_object.repeated().at_least(1).flatten(),
        )
        .padded()
        .chain(token("}", ShExMLToken::CurlEnd))
        .padded();

    let with_graph = graph_node
        .chain(
            token("[", ShExMLToken::SqBrackStart)
                .repeated()
                .at_least(2)
                .at_most(2),
        )
        .chain::<ShExMLToken, _, _>(
            single_shape.clone().repeated().at_least(1).flatten(),
        )
        .padded()
        .chain::<ShExMLToken, _, _>(
            token("]", ShExMLToken::SqBrackEnd)
                .repeated()
                .at_least(2)
                .at_most(2),
        );

    (with_graph.or(single_shape))
        .padded()
        .repeated()
        .at_least(1)
        .flatten()
}

fn shape_object() -> t!(Vec<ShExMLToken>) {
    let language_tag = token("@", ShExMLToken::AtSymb).chain(
        shape_node_expression().or(pn_char().repeated().at_least(1).map(
            |chars| vec![ShExMLToken::LangTag(chars.into_iter().collect())],
        )),
    );

    let data_type_static = prefix_namespace().chain::<ShExMLToken, _, _>(
        pn_char().repeated().at_least(1).map(|chars| {
            vec![ShExMLToken::PrefixLN(chars.into_iter().collect())]
        }),
    );

    let data_type = choice((
        prefix_namespace().chain(shape_node_expression()),
        shape_node_expression(),
        data_type_static,
    ));

    let shape_link = token("@", ShExMLToken::AtSymb)
        .chain(token(":", ShExMLToken::PrefixSep))
        .chain(ident());

    let object = choice((
        shape_link,
        prefix_namespace().chain::<ShExMLToken, _, _>(shape_node_expression()),
        shape_node_expression(),
    ))
    .padded();

    object
        .padded()
        .chain::<ShExMLToken, _, _>(language_tag.or(data_type).or_not())
        .padded()
}

fn shape_node_expression() -> t!(Vec<ShExMLToken>) {
    let matching = shape_sub_ident()
        .padded()
        .chain(token("MATCHING ", ShExMLToken::Matching))
        .chain(ident());

    let if_block = shape_sub_ident()
        .padded()
        .chain(token("IF ", ShExMLToken::If))
        .chain::<ShExMLToken, _, _>(shape_function_application());

    token("[", ShExMLToken::SqBrackStart)
        .chain(choice((
            matching,
            if_block,
            shape_function_application(),
            shape_sub_ident(),
        )))
        .chain(token("]", ShExMLToken::SqBrackEnd))
}

fn shape_sub_ident() -> t!(Vec<ShExMLToken>) {
    ident().chain(
        token(".", ShExMLToken::Dot)
            .chain(ident())
            .repeated()
            .flatten(),
    )
}

fn shape_function_application() -> t!(Vec<ShExMLToken>) {
    shape_sub_ident()
        .chain(token("(", ShExMLToken::BrackStart))
        .chain::<ShExMLToken, _, _>(shape_sub_ident())
        .chain::<ShExMLToken, _, _>(
            token(",", ShExMLToken::Comma)
                .padded()
                .chain(shape_sub_ident())
                .repeated()
                .flatten(),
        )
        .chain(token(")", ShExMLToken::BrackEnd))
}

pub fn functions() -> t!(Vec<ShExMLToken>) {
    let function_tag = token("FUNCTIONS", ShExMLToken::Function);
    let function_ident = ident().padded();

    let protocol = protocol().padded().map(ShExMLToken::FunctionLang);
    let uri = protocol_iri_ref().or(path().map(|st| ShExMLToken::URI(st)));

    let function_exp = token("<", ShExMLToken::AngleStart)
        .chain(protocol.chain(uri))
        .chain(token(">", ShExMLToken::AngleEnd).padded());
    (function_tag.chain(function_ident).chain(function_exp))
        .padded()
        .repeated()
        .flatten()
}

pub fn autoincrements() -> t!(Vec<ShExMLToken>) {
    let aut_inc_tag = token("AUTOINCREMENT", ShExMLToken::AutoIncrement);
    let ident = ident().padded();
    let prefix_str = text::ident::<char, _>()
        .map(|chars: String| ShExMLToken::AutoIncPrefix(chars))
        .delimited_by(just('"'), just('"'))
        .then_ignore(just('+').padded());

    let sufix_str = just('+').padded().ignore_then(
        text::ident()
            .map(|chars: String| ShExMLToken::AutoIncSuffix(chars))
            .delimited_by(just('"'), just('"')),
    );

    let start_inc = text::digits::<char, _>(10)
        .padded()
        .map(|digit: String| ShExMLToken::AutoIncStart(digit.parse().unwrap()));

    let end_inc =
        just("to").padded().ignore_then(text::digits(10).map(
            |digit: String| ShExMLToken::AutoIncEnd(digit.parse().unwrap()),
        ));

    let step_inc =
        just("by").padded().ignore_then(text::digits(10).map(
            |digit: String| ShExMLToken::AutoIncStep(digit.parse().unwrap()),
        ));

    let aut_inc_exp = prefix_str
        .or_not()
        .chain::<ShExMLToken, _, _>(start_inc)
        .chain::<ShExMLToken, _, _>(end_inc.or_not())
        .chain::<ShExMLToken, _, _>(step_inc.or_not())
        .chain::<ShExMLToken, _, _>(sufix_str.or_not());

    let auto_inc_exp_delim = token("<", ShExMLToken::AngleStart)
        .chain::<ShExMLToken, Vec<_>, _>(aut_inc_exp)
        .chain(token(">", ShExMLToken::AngleEnd));

    (aut_inc_tag.chain(ident).chain(auto_inc_exp_delim))
        .padded()
        .repeated()
        .flatten()
}

pub fn matchers() -> t!(Vec<ShExMLToken>) {
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

    (mat_tag.chain(mat_ident).chain(
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
    ))
    .padded()
    .repeated()
    .flatten()
}

pub fn expressions() -> t!(Vec<ShExMLToken>) {
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
    (expressiont_tag.chain(exp_ident).chain(exp_inner))
        .padded()
        .repeated()
        .at_least(1)
        .flatten()
}

pub fn iterators() -> t!(Vec<ShExMLToken>) {
    let header = iterator_header().padded();

    (recursive(|iter| {
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
            .chain(
                token("}", ShExMLToken::BrackEnd)
                    .map(|tok| vec![tok])
                    .or_not(),
            )
            .padded()
    }))
    .padded()
    .repeated()
    .at_least(1)
    .flatten()
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

    iterator_tag
        .chain(iterator_name)
        .chain(iter_query_pair)
        .repeated()
        .at_least(1)
        .flatten()
}

pub fn sources() -> t!(Vec<ShExMLToken>) {
    let source_tag = token("SOURCE", ShExMLToken::Source);
    let source_name = ident().padded();
    let source_iri = token("<", ShExMLToken::AngleStart)
        .chain(protocol_iri_ref().or(path().map(|st| ShExMLToken::URI(st))))
        .chain(token(">", ShExMLToken::AngleEnd));
    source_tag
        .chain(source_name)
        .chain(source_iri)
        .padded()
        .repeated()
        .at_least(1)
        .flatten()
}

pub fn ident() -> t!(ShExMLToken) {
    pn_char()
        .repeated()
        .at_least(1)
        .map(|v| ShExMLToken::Ident(v.into_iter().collect()))
}

pub fn prefixes() -> t!(Vec<ShExMLToken>) {
    let prefix_tag = token("PREFIX", ShExMLToken::Prefix);
    let pname = prefix_namespace();

    prefix_tag
        .chain(pname)
        .chain(
            token("<", ShExMLToken::AngleStart)
                .chain(protocol_iri_ref())
                .chain(token(">", ShExMLToken::AngleEnd)),
        )
        .padded()
        .repeated()
        .at_least(1)
        .flatten()
}

fn prefix_namespace() -> t!(Vec<ShExMLToken>) {
    pn_prefix()
        .or_not()
        .then_ignore(just(":"))
        .padded()
        .flatten()
        .map(|pname_vec| {
            if pname_vec.is_empty() {
                vec![ShExMLToken::BasePrefix, ShExMLToken::PrefixSep]
            } else {
                let prefix = pname_vec.into_iter().collect();
                vec![ShExMLToken::PrefixNS(prefix), ShExMLToken::PrefixSep]
            }
        })
}

fn protocol() -> t!(String) {
    filter(|c: &char| c.is_ascii_alphabetic())
        .repeated()
        .at_least(1)
        .chain(just(':'))
        .map(|c| c.into_iter().collect())
}

fn path() -> t!(String) {
    none_of("<>\"{}|^`\\[]")
        .repeated()
        .at_least(1)
        .map(|e| e.into_iter().collect())
}

fn protocol_iri_ref() -> t!(ShExMLToken) {
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
