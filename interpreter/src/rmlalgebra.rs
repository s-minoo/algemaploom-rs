use operator::Operator;

use crate::rml_model::term_map::{
    GraphMap, PredicateObjectMap, SubjectMap, TriplesMap,
};
use crate::rml_model::Document;

pub fn translate_to_algebra(doc: Document) -> Vec<Operator> {
    for tm in doc.triples_maps {
        let source = translate_triples_map(tm);
    }
    todo!()
}

pub fn translate_term_maps(
    sm: &SubjectMap,
    poms: &Vec<PredicateObjectMap>,
    gm: &Option<GraphMap>,
) -> Operator {
    
    todo!()
}


// TODO: Implement mapping operator parsing properly!  <18-04-23, Sitt Min OO> //
// TODO: Implement target opeartor parsing properly! <18-04-23, Sitt Min OO> //
fn translate_triples_map(tm: TriplesMap) -> Operator {
    let source_op = Operator::SourceOp(
        tm.logical_source.into(),
        Box::new(translate_term_maps(
            &tm.subject_map,
            &tm.po_maps,
            &tm.graph_map,
        )),
    );
    todo!()
}
