use std::convert::Infallible;
use std::io;

use sophia_term::TermError;

#[derive(Debug)]
pub enum ParseError {
    IOErrorStr(String),
    IOError(io::Error),
    TermError(TermError),
    GenericError(String),
    ExtensionError(String),
    Infallible,
}

impl From<Infallible> for ParseError {
    fn from(_: Infallible) -> Self {
        ParseError::Infallible
    }
}

impl From<TermError> for ParseError {
    fn from(value: TermError) -> Self {
        ParseError::TermError(value)
    }
}

impl From<io::Error> for ParseError {
    fn from(value: io::Error) -> Self {
        ParseError::IOError(value)
    }
}
