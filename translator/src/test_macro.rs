#[macro_export]
macro_rules! test_case {($fname:expr) => (
  concat!(env!("CARGO_MANIFEST_DIR"), "/resources/tests/", $fname) // assumes Linux ('/')!
)}

#[macro_export]
macro_rules! load_graph {
    ($fname:expr) => {{
        let path = test_case!($fname);
        let pathbuf = PathBuf::from(path);
        let bread = BufReader::new(File::open(pathbuf)?);
        load_graph_bread(bread)
    }};
}

#[macro_export]
macro_rules! import_test_mods {
    () => {
        use std::fs::File;
        use std::io::BufReader;
        use std::path::PathBuf;

        use sophia_api::graph::Graph;
        use sophia_api::triple::Triple;

        use interpreter::extractors::io::load_graph_bread;
        use interpreter::extractors::{ExtractorResult, FromVocab, TermMapExtractor};
        use crate::{load_graph, test_case};
    };
}
