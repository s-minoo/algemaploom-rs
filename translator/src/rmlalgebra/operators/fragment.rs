use std::collections::HashMap;

use operator::{Fragmenter, Operator};

use super::RMLTranslator;
use crate::rmlalgebra::types::Triples;
#[derive(Debug, Clone)]
pub struct FragmentTranslator<'a> {
    pub lt_triples_map: &'a HashMap<String, Vec<Triples<'a>>>,
}

impl<'a> RMLTranslator<Option<Fragmenter>> for FragmentTranslator<'a> {
    fn translate(self) -> Option<Fragmenter> {
        translate_fragment_op_from_lts(self.lt_triples_map)
    }
}

fn translate_fragment_op_from_lts_str(
    lt_triples_map: &HashMap<String, Vec<Triples>>,
    from_fragment: &str,
) -> Option<Fragmenter> {
    let target_lt_ids = lt_triples_map.keys();

    let to: Vec<String> = target_lt_ids.map(|id| id.clone()).collect();

    if to.len() == 1 && to.iter().next() == Some(&from_fragment.to_string()) {
        return None;
    }

    Some(Fragmenter {
        from: from_fragment.to_string(),
        to,
    })
}

fn translate_fragment_op_from_lts(
    lt_triples_map: &HashMap<String, Vec<Triples>>,
) -> Option<Fragmenter> {
    translate_fragment_op_from_lts_str(lt_triples_map, "default")
}
