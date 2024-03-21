use std::collections::HashMap;

use shexml_interpreter::{Object, ShExMLQuads, Subject};

pub struct IndexVariableTerm<'a> {
    pub subject_variable_index: HashMap<&'a Subject, String>,
    pub object_variable_index:  HashMap<&'a Object, String>,
}

pub fn variablelize_quads(quads: &ShExMLQuads<'_>) -> IndexVariableTerm<'_> {
    let mut subject_variable_index = HashMap::new();
    let mut object_variable_index = HashMap::new();

    for (idx, (subj, _, obj, graph)) in quads.iter().enumerate() {
        if !subject_variable_index.contains_key(subj) {
            let subject_variable = format!("{}_sm_{}", graph.local, idx);
            subject_variable_index.insert(*subj, subject_variable);
        }
        let object_variable = format!("{}_om_{}", graph.local, idx);

        object_variable_index.insert(*obj, object_variable);
    }

    IndexVariableTerm {
        subject_variable_index,
        object_variable_index,
    }
}
