pub mod r#type;
mod token;
mod lexer;

use std::path::Path;

use r#type::ShExMLDocument;


pub fn parse_file<P: AsRef<Path>>(file_path: P) -> ShExMLDocument<'static> {
    todo!()
}

