pub mod formats;
pub mod query;
pub mod r2rml;
pub mod rdf;
pub mod rml;
pub mod xsd;

pub type PAIR<'a> = (&'a str, &'a str);

pub trait ToString {
    fn to_string(self) -> String;
}

impl<'a> ToString for PAIR<'a> {
    fn to_string(self) -> String {
        format!("{}{}", self.0, self.1)
    }
}
