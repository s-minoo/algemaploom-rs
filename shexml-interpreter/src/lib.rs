mod lexer;
mod parser;
mod token;

use std::path::Path;

use crate::parser::r#type::ShExMLDocument;

pub fn parse_file<P: AsRef<Path>>(file_path: P) -> ShExMLDocument {
    todo!()
}
