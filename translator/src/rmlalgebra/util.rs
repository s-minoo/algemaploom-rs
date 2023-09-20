use std::collections::{HashMap, HashSet};

use interpreter::rml_model::source_target::LogicalTarget;
use interpreter::rml_model::Document;
use operator::Target;

use super::types::TermMapEnum;

pub fn file_target(count: usize) -> Target {
    let mut config = HashMap::new();
    config.insert("path".to_string(), format!("{}_output.nt", count));
    Target {
        configuration: config,
        target_type:   operator::IOType::File,
        data_format:   operator::formats::DataFormat::NTriples,
    }
}

pub fn partition_by_logical_target(
    tm: &Document,
) -> HashMap<String, Vec<TermMapEnum>> {
    // TODO:  <20-09-23, yourname> //
    todo!()
}

pub fn generate_logtarget_map(
    doc: &Document,
) -> HashMap<String, &LogicalTarget> {
    let logical_targets =
        doc.triples_maps.iter().fold(HashSet::new(), |mut set, tm| {
            set.extend(&tm.subject_map.tm_info.logical_targets);

            tm.po_maps.iter().for_each(|pom| {
                let pms_lts = pom
                    .predicate_maps
                    .iter()
                    .flat_map(|pm| &pm.tm_info.logical_targets);

                set.extend(pms_lts);

                let oms_lts = pom
                    .object_maps
                    .iter()
                    .flat_map(|om| &om.tm_info.logical_targets);

                set.extend(oms_lts);
            });

            set
        });

    logical_targets
        .into_iter()
        .map(|lt| (lt.identifier.clone(), lt))
        .collect()
}

pub fn generate_variable_map(doc: &Document) -> HashMap<String, String> {
    let mut result_map: HashMap<String, String> = HashMap::new();

    for (tm_idx, triples_map) in doc.triples_maps.iter().enumerate() {
        let tm_prefix = format!("tm{}", tm_idx);

        let subject_map = &triples_map.subject_map;

        result_map.insert(
            subject_map.tm_info.identifier.clone(),
            format!("{}_sm", tm_prefix),
        );

        for (pom_idx, pom) in triples_map.po_maps.iter().enumerate() {
            for (pm_idx, pm) in pom.predicate_maps.iter().enumerate() {
                result_map.insert(
                    pm.tm_info.identifier.clone(),
                    format!("{}_p{}_{}", tm_prefix, pom_idx, pm_idx),
                );
            }

            for (om_idx, om) in pom.object_maps.iter().enumerate() {
                result_map.insert(
                    om.tm_info.identifier.clone(),
                    format!("{}_o{}_{}", tm_prefix, pom_idx, om_idx),
                );
            }
        }
    }

    result_map
}
