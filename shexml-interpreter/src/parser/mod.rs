mod tests;
pub mod r#type;
use std::collections::HashMap;

use chumsky::prelude::*;
use chumsky::Parser;

use self::r#type::*;
use crate::lexer::token::*;

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

pub fn shexml() -> t!(ShExMLDocument) {
    prefixes()
        .then(sources())
        .then(iterators())
        .map(|((prefixes, sources), iters)| (prefixes, sources, iters))
        .then(expressions())
        .then(graph_shapes())
        .map(
            |(((prefixes, sources, iterators), expressions), graph_shapes)| {
                let mut matchers = Vec::new();
                let mut auto_increments = Vec::new();
                let mut expression_stmts = Vec::new();
                let mut functions = Vec::new();

                for expr in expressions {
                    match expr {
                        ExpressionEnum::ExpressionStmt(stmt) => {
                            expression_stmts.push(stmt)
                        }
                        ExpressionEnum::MatcherExp(matcher) => {
                            matchers.push(matcher)
                        }
                        ExpressionEnum::AutoIncrementExp(auto_increment) => {
                            auto_increments.push(auto_increment)
                        }
                        ExpressionEnum::FunctionExp(function) => {
                            functions.push(function)
                        }
                    }
                }

                ShExMLDocument {
                    prefixes,
                    sources,
                    iterators,
                    expression_stmts,
                    matchers,
                    auto_increments,
                    functions,
                    graph_shapes,
                }
            },
        )
}

fn shape_ident() -> t!(ShapeIdent) {
    let pn_ln = select! {
        ShExMLToken::ShapeNode{prefix, local} => (prefix, local)
    };

    pn_ln
        .map(|(prefix, local)| {
            let prefix = if prefix.is_empty() {
                PrefixNameSpace::BasePrefix
            } else {
                PrefixNameSpace::NamedPrefix(prefix)
            };

            ShapeIdent { prefix, local }
        })
        .labelled("parser:shape_ident")
}

fn graph_shapes() -> t!(Vec<GraphShapes>) {
    let graph_ident = shape_ident();
    let graph = graph_ident
        .then_ignore(
            just(ShExMLToken::SqBrackStart)
                .repeated()
                .at_least(2)
                .at_most(2),
        )
        .then(shapes())
        .then_ignore(
            just(ShExMLToken::SqBrackEnd)
                .repeated()
                .at_least(2)
                .at_most(2),
        )
        .map(|(ident, shapes)| GraphShapes { ident, shapes });

    graph
        .or(shapes().map(|shapes| {
            GraphShapes {
                ident: ShapeIdent::base(),
                shapes,
            }
        }))
        .repeated()
        .at_least(1)
        .labelled("parser:graph_shapes")
}

fn shapes() -> t!(Vec<Shape>) {
    let shape_expr = just(ShExMLToken::SqBrackStart)
        .ignore_then(shape_expression())
        .then_ignore(just(ShExMLToken::SqBrackEnd));

    let shape_expr_prefix = select! {
        ShExMLToken::BasePrefix => PrefixNameSpace::BasePrefix,
        ShExMLToken::PrefixNS(prefix) => PrefixNameSpace::NamedPrefix(prefix),
    };

    let prefix_shape_pair =
        shape_expr_prefix.then_ignore(just(ShExMLToken::PrefixSep));

    let subj_prefix = prefix_shape_pair.clone().then(shape_expr.clone());

    let obj_prefix = prefix_shape_pair.or_not().then(shape_expr);

    let shape_ident = shape_ident();
    let predicate = select! {
        ShExMLToken::ShapeTerm{prefix, local} =>{
            let mut p_ns = PrefixNameSpace::BasePrefix;
            if !prefix.is_empty(){

                p_ns = PrefixNameSpace::NamedPrefix(prefix);
            }
        Predicate{ prefix: p_ns, local: local}
        }
    };
    shape_ident
        .clone()
        .then(
            subj_prefix
                .clone()
                .map(|(prefix, expression)| Subject { prefix, expression }),
        )
        .then_ignore(just(ShExMLToken::CurlStart))
        .then(
            predicate
                .clone()
                .then(
                    obj_prefix.map(|(prefix, expression)| {
                        Object { prefix, expression }
                    }),
                )
                .then_ignore(just(ShExMLToken::PredicateSplit))
                .repeated()
                .at_least(1),
        )
        .then_ignore(just(ShExMLToken::CurlEnd))
        .map(|((ident, subject), pred_obj_pairs)| {
            let pred_obj_pairs: Vec<(Predicate, Object)> = pred_obj_pairs;

            Shape {
                ident,
                subject,
                pred_obj_pairs: pred_obj_pairs.into_iter().collect(),
            }
        })
        .repeated()
        .at_least(1)
        .labelled("parser:shapes")
}

fn shape_expression() -> t!(ShapeExpression) {
    let reference_expr = unfold_token_value!(Ident)
        .then(
            shex_just!(ShExMLToken::Dot)
                .ignore_then(unfold_token_value!(Ident))
                .or_not(),
        )
        .map(|(expr_ident, field)| ShapeReference { expr_ident, field });

    let func_params = reference_expr
        .clone()
        .separated_by(just(ShExMLToken::Comma));
    let func_expr = reference_expr
        .clone()
        .then(func_params.delimited_by(
            just(ShExMLToken::BrackStart),
            just(ShExMLToken::BrackEnd),
        ))
        .map(|(fun_method_ident, params_idents)| {
            ShapeExpression::Function {
                fun_method_ident,
                params_idents,
            }
        });

    let conditional_expr = reference_expr
        .clone()
        .then_ignore(just(ShExMLToken::If))
        .then(func_expr.clone())
        .map(|(reference, function)| {
            ShapeExpression::Conditional {
                reference,
                conditional_expr: Box::new(function),
            }
        });

    choice((
        conditional_expr,
        func_expr,
        reference_expr.map(ShapeExpression::Reference),
    ))
    .labelled("parser:shape_expression")
}

fn expressions() -> t!(Vec<ExpressionEnum>) {
    (expression_stmt()
        .or(matcher())
        .or(function())
        .or(auto_increment()))
    .repeated()
    .at_least(1)
    .labelled("parser:expressions")
}

fn function() -> t!(ExpressionEnum) {
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
            let function = Function {
                ident: ident.clone(),
                lang_type,
                uri,
            };
            ExpressionEnum::FunctionExp(function)
        })
        .labelled("parser:function")
}

fn auto_increment() -> t!(ExpressionEnum) {
    let auto_inc_ident_exp = unfold_token_value!(Ident)
        .then_ignore(just(ShExMLToken::AngleStart))
        .then(
            unfold_token_value!(AutoIncPrefix)
                .or_not()
                .then(unfold_token_value!(AutoIncStart))
                .then(unfold_token_value!(AutoIncEnd).or_not())
                .then(unfold_token_value!(AutoIncStep).or_not())
                .then(unfold_token_value!(AutoIncSuffix).or_not()),
        )
        .then_ignore(just(ShExMLToken::AngleEnd))
        .map(|(ident, ((((prefix, start), end), step), suffix))| {
            let auto_increment = AutoIncrement {
                ident: ident.clone(),
                start,
                prefix,
                suffix,
                end,
                step,
            };
            ExpressionEnum::AutoIncrementExp(auto_increment)
        });

    just(ShExMLToken::AutoIncrement)
        .ignore_then(auto_inc_ident_exp)
        .labelled("parser:auto_increment")
}

fn matcher() -> t!(ExpressionEnum) {
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
            let matcher = Matcher {
                ident: ident.clone(),
                rename_map,
            };

            ExpressionEnum::MatcherExp(matcher)
        })
        .labelled("parser:matcher")
}

fn exp_reference_ident() -> t!(ExpressionReferenceIdent) {
    unfold_token_value!(Ident)
        .then(
            just(ShExMLToken::Dot)
                .ignore_then(unfold_token_value!(Ident))
                .repeated()
                .at_least(1)
                .at_most(2)
                .map(|idents| {
                    let iterator = idents[0].clone();
                    let field = idents.get(1).map(Clone::clone);

                    (iterator, field)
                }),
        )
        .map(|(source_ident, (iterator_ident, field))| {
            ExpressionReferenceIdent {
                source_ident,
                iterator_ident,
                field,
            }
        })
        .labelled("parser:exp_ident")
}

fn expression_stmt() -> t!(ExpressionEnum) {
    just::<ShExMLToken, _, Simple<ShExMLToken>>(ShExMLToken::Expression)
        .ignore_then(unfold_token_value!(Ident))
        .then_ignore(just(ShExMLToken::AngleStart))
        .then(exp_string_op().or(exp_join_union()))
        .then_ignore(just(ShExMLToken::AngleEnd))
        .map(|(ident, expression)| {
            let stmt = ExpressionStmt {
                ident,
                expr_enum: expression,
            };
            ExpressionEnum::ExpressionStmt(stmt)
        })
        .labelled("parser:expression_stmt")
}

fn exp_join_union() -> t!(ExpressionStmtEnum) {
    let basic_expression = exp_reference_ident()
        .map(|path| ExpressionStmtEnum::Basic { reference: path });
    basic_expression
        .clone()
        .then(
            just(ShExMLToken::Union)
                .to(ExpressionStmtEnum::Union as fn(_, _) -> _)
                .or(just(ShExMLToken::Join)
                    .to(ExpressionStmtEnum::Join as fn(_, _) -> _)),
        )
        .repeated()
        .then(basic_expression)
        .foldr(|(lhs, op), rhs| op(Box::new(lhs), Box::new(rhs)))
        .labelled("parser:exp_join_union")
}

fn exp_string_op() -> t!(ExpressionStmtEnum) {
    exp_reference_ident()
        .then(unfold_token_value!(StringSep))
        .then(exp_reference_ident())
        .map(|((left_reference, concate_string), right_reference)| {
            ExpressionStmtEnum::ConcatenateString {
                left_reference,
                concate_string,
                right_reference,
            }
        })
        .labelled("parser:exp_string_op")
}

fn sources() -> t!(Vec<Source>) {
    let protocol = select! {
        ShExMLToken::File => SourceType::File,
        ShExMLToken::HTTP => SourceType::HTTP,
        ShExMLToken::HTTPS => SourceType::HTTPS,
        ShExMLToken::JDBC(jdbc_type) => SourceType::JDBC(jdbc_type),
    };

    let protocol_uri = protocol.then(unfold_token_value!(URI));

    just(ShExMLToken::Source)
        .ignore_then(unfold_token_value!(Ident))
        .then(protocol_uri.delimited_by(
            just(ShExMLToken::AngleStart),
            just(ShExMLToken::AngleEnd),
        ))
        .map(|(ident, (source_type, uri))| {
            Source {
                ident,
                uri,
                source_type,
            }
        })
        .repeated()
        .at_least(1)
        .labelled("parser:sources")
}

fn iterators() -> t!(Vec<Iterator>) {
    let fields = field().repeated().at_least(1);

    recursive(|recurs| {
        just::<ShExMLToken, _, Simple<ShExMLToken>>(ShExMLToken::Iterator)
            .ignore_then(unfold_token_value!(Ident))
            .then(
                unfold_token_value!(IteratorType)
                    .map(|iter| iter.parse::<IteratorType>().unwrap())
                    .or_not()
                    .then(unfold_token_value!(IteratorQuery))
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
            .then(recurs.repeated())
            .then_ignore(just(ShExMLToken::BrackEnd).or_not())
            .map(|((ident, iter_type, query, fields), nested_iterator)| {
                Iterator {
                    ident,
                    query,
                    iter_type,
                    fields,
                    nested_iterator,
                }
            })
    })
    .repeated()
    .at_least(1)
    .labelled("parser:iterators")
}

fn field() -> t!(Field) {
    let field_type = select! {
        ShExMLToken::PushField => FieldType::Push,
        ShExMLToken::Field => FieldType::Normal,
        ShExMLToken::PopField => FieldType::Pop,
        _ => FieldType::Normal,
    };

    field_type
        .then(unfold_token_value!(Ident))
        .then(unfold_token_value!(FieldQuery))
        .map(|((field_type, name), query)| {
            Field {
                ident: name,
                query,
                field_type,
            }
        })
        .labelled("parser:field_type")
}

fn prefixes() -> t!(Vec<Prefix>) {
    let prefix_ns = select! {
        ShExMLToken::PrefixNS(ns) => PrefixNameSpace::NamedPrefix(ns),
        ShExMLToken::BasePrefix => PrefixNameSpace::BasePrefix,

    };

    just(ShExMLToken::Prefix)
        .ignore_then(prefix_ns)
        .then_ignore(just(ShExMLToken::PrefixSep))
        .then(unfold_token_value!(URI).delimited_by(
            just(ShExMLToken::AngleStart),
            just(ShExMLToken::AngleEnd),
        ))
        .map(|(prefix, uri)| Prefix { prefix, uri })
        .repeated()
        .at_least(1)
        .labelled("parser:prefixes")
}
