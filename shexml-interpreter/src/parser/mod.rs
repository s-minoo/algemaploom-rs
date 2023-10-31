pub mod r#type;
use chumsky::prelude::*;

use self::r#type::*;
use crate::token::*;

macro_rules! t {
    ($t:ty) => {
        impl Parser<ShExMLToken, $t, Error = Simple<ShExMLToken>> + Clone
    };
}

fn ident_to_string() -> t!(String) {
    select! {
        ShExMLToken::Ident(ident) => ident
    }
}

fn sources() -> t!(Vec<Source>) {
    let uri_to_string = select! {
        ShExMLToken::URI(uri) => uri
    };

    just(ShExMLToken::Source)
        .ignore_then(ident_to_string())
        .then(uri_to_string.delimited_by(
            just(ShExMLToken::AngleStart),
            just(ShExMLToken::AngleEnd),
        ))
        .map(|(ident, uri)| Source { name: ident, uri })
        .repeated()
        .at_least(1)
}

fn fields() -> t!(Vec<Field>) {
    todo()
}

fn iterators() -> t!(Vec<Iterator>) {
    todo()
}

fn prefixes() -> t!(Vec<PrefixNameSpace>) {
    let string_val_parser = select! {
        ShExMLToken::PrefixNS(ns) => ns,
        ShExMLToken::BasePrefix => "".to_string(),
        ShExMLToken::URI(uri) => uri,

    };

    just(ShExMLToken::Prefix)
        .ignore_then(string_val_parser)
        .then_ignore(just(ShExMLToken::AngleStart))
        .then(string_val_parser)
        .then_ignore(just(ShExMLToken::AngleStart))
        .map(|(prefix, local)| PrefixNameSpace { prefix, local })
        .repeated()
        .at_least(1)
}
