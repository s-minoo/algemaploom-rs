use std::path::PathBuf;

#[macro_export]
macro_rules! test_case {($fname:expr) => (
  concat!(env!("CARGO_MANIFEST_DIR"), "/resources/tests/", $fname) // assumes Linux ('/')!
)}

#[macro_export]
macro_rules! load_graph {
    ($fname:expr) => {
        {
            let path =  test_case!($fname);
            let pathbuf  = PathBuf::from(path);
            let bread = BufReader::new(File::open(pathbuf)?);
            load_graph_bread(bread)
        }

    };
}





