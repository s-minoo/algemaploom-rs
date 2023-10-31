mod token;
mod lexer;
mod parser;

use std::path::Path;

use r#type::ShExMLDocument;


pub fn parse_file<P: AsRef<Path>>(file_path: P) -> ShExMLDocument<'static> {
    todo!()
}

