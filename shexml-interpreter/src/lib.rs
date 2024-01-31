mod lexer;
mod parser;

use std::path::Path;

use chumsky::Parser;

use crate::parser::r#type::ShExMLDocument;

pub fn parse_file<P: AsRef<Path>>(file_path: P) -> ShExMLDocument {
    todo!()
}

pub fn parse_string(shexml_doc: String) -> ShExMLDocument {
    let (tokens_opt, errors) = lexer::shexml().parse_recovery(shexml_doc);

    todo!()
}
