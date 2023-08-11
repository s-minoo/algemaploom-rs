use std::path::PathBuf;


use super::Source;

pub mod csv;

pub trait FileSource: Source {
    fn file(&self) -> PathBuf;
    fn close(&mut self); 

}
