use rml_interpreter::test_case;
use shexml_interpreter::errors::ShExMLResult;

use super::*;

macro_rules! test_case {
    ($fname:expr) => {
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/resources/test/shexml/",
            $fname
        )
    };
}

#[test]
fn source_translate_test() -> ShExMLResult<()> {
    let simple_shexml = test_case!("sample.shexml");
    let shexml_doc = shexml_interpreter::parse_file(simple_shexml)?;
    let source_translator = ShExMLSourceTranslator {
        document: &shexml_doc,
    };

    let alge_source = source_translator.translate();

    for (source_ident, (source, expr_ident)) in alge_source.iter(){

        println!("Source id: {:?}", source_ident); 
        println!("Source: {:#?}", source); 
        println!("Expr idents: {:#?}", expr_ident);

    }

    Ok(())
}
