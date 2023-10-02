use std::collections::{HashMap, HashSet};
use std::vec;

use interpreter::rml_model::source_target::LogicalTarget;
use interpreter::rml_model::term_map::SubjectMap;
use interpreter::rml_model::{Document, PredicateObjectMap};
use operator::Target;

use super::types::{RefPOM, Triples};

pub fn file_target(count: usize) -> Target {
    let mut config = HashMap::new();
    config.insert("path".to_string(), format!("{}_output.nt", count));
    Target {
        configuration: config,
        target_type:   operator::IOType::File,
        data_format:   operator::formats::DataFormat::NTriples,
    }
}

pub fn generate_lt_tm_map_from_doc(
    doc: &Document,
) -> HashMap<String, Vec<Triples>> {
    let mut result = HashMap::new();
    for tm in &doc.triples_maps {
        result
            .extend(generate_lt_tm_map_from_spo(&tm.subject_map, &tm.po_maps));
    }

    result
}

pub fn generate_lt_tm_map_from_spo<'a>(
    sm: &'a SubjectMap,
    poms: &'a [PredicateObjectMap],
) -> HashMap<String, Vec<Triples<'a>>> {
    let mut result = HashMap::new();
    let mut sm_lts = &sm.tm_info.logical_targets;
    let hash_set = vec![LogicalTarget::default()].into_iter().collect();
    if sm_lts.is_empty() {
        sm_lts = &hash_set;
    }

    sm_lts.iter().for_each(|lt| {
        let triples = Triples {
            sm,
            poms: poms.iter().map(|pom| pom.into()).collect(),
        };
        update_lt_map(&mut result, lt, triples);
    });

    for pom in poms {
        let oms = &pom.object_maps;
        let pms = &pom.predicate_maps;

        for pm in pms {
            pm.tm_info.logical_targets.iter().for_each(|lt| {
                let ref_pom = RefPOM {
                    pm: vec![pm],
                    om: oms.iter().map(|om| om.into()).collect(),
                };
                let triples = Triples {
                    sm,
                    poms: vec![ref_pom],
                };

                update_lt_map(&mut result, lt, triples);
            });
        }

        for om in oms {
            om.tm_info.logical_targets.iter().for_each(|lt| {
                let ref_pom = RefPOM {
                    pm: pms.iter().map(|pm| pm.into()).collect(),
                    om: vec![om],
                };

                let triples = Triples {
                    sm,
                    poms: vec![ref_pom],
                };

                update_lt_map(&mut result, lt, triples);
            })
        }
    }

    result
}

fn update_lt_map<'a>(
    result: &mut HashMap<String, Vec<Triples<'a>>>,
    lt: &LogicalTarget,
    triples: Triples<'a>,
) {
    if let Some(mut existing_vec) =
        result.insert(lt.identifier.clone(), vec![triples.clone()])
    {
        existing_vec.push(triples);
    }
}

pub fn generate_logtarget_map(doc: &Document) -> HashMap<String, Target> {
    let logical_targets =
        doc.triples_maps.iter().fold(HashSet::new(), |mut set, tm| {
            set.extend(tm.subject_map.tm_info.logical_targets.clone());

            tm.po_maps.iter().for_each(|pom| {
                let pms_lts = pom
                    .predicate_maps
                    .iter()
                    .flat_map(|pm| pm.tm_info.logical_targets.clone());

                set.extend(pms_lts);

                let oms_lts = pom
                    .object_maps
                    .iter()
                    .flat_map(|om| om.tm_info.logical_targets.clone());

                set.extend(oms_lts);
            });

            set
        });

    logical_targets
        .into_iter()
        .map(|lt| (lt.identifier.clone(), lt.into()))
        .collect()
}

pub fn generate_variable_map(doc: &Document) -> HashMap<String, String> {
    let mut result_map: HashMap<String, String> = HashMap::new();

    for (tm_idx, triples_map) in doc.triples_maps.iter().enumerate() {
        let tm_prefix = format!("tm{}", tm_idx);
        result_map.insert(triples_map.identifier.clone(), tm_prefix.clone());

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
