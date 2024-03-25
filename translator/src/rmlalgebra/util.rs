use std::collections::{HashMap, HashSet};
use std::vec;

use operator::Target;
use rml_interpreter::rml_model::source_target::LogicalTarget;
use rml_interpreter::rml_model::term_map::{SubjectMap, TermMapInfo};
use rml_interpreter::rml_model::{Document, PredicateObjectMap};

use super::types::{Quads, RefPOM, Triples};

pub fn extract_gm_tm_infos<'a>(
    sm: &'a SubjectMap,
    poms: &'a [PredicateObjectMap],
) -> Vec<&'a TermMapInfo> {
    let mut result = vec![];

    result.extend(sm.graph_maps.iter().map(|gm| &gm.tm_info));

    poms.iter().for_each(|pom| {
        result.extend(pom.predicate_maps.iter().map(|pm| &pm.tm_info));
        result.extend(pom.object_maps.iter().map(|om| &om.tm_info));
    });

    result
}

pub fn extract_tm_infos_from_poms(
    poms: &[PredicateObjectMap],
) -> Vec<&TermMapInfo> {
    poms.iter()
        .flat_map(|pom| {
            let mut tm_infos: Vec<_> =
                pom.predicate_maps.iter().map(|pm| &pm.tm_info).collect();
            let om_infos = pom.object_maps.iter().map(|om| &om.tm_info);

            let gm_infos = pom.graph_maps.iter().map(|gm| &gm.tm_info);
            tm_infos.extend(om_infos);
            tm_infos.extend(gm_infos);
            tm_infos
        })
        .collect()
}

pub fn generate_lt_quads_from_doc(
    doc: &Document,
) -> HashMap<String, Vec<Quads>> {
    let mut result = HashMap::new();
    for tm in &doc.triples_maps {
        result.extend(generate_lt_quads_from_spo(&tm.subject_map, &tm.po_maps));
    }

    result
}

pub fn generate_lt_quads_from_spo<'a>(
    sm: &'a SubjectMap,
    poms: &'a [PredicateObjectMap],
) -> HashMap<String, Vec<Quads<'a>>> {
    let mut result = HashMap::new();
    let sm_lts = &sm.tm_info.logical_targets;
    if sm_lts.is_empty() {
        panic!("Subject map's logical target is empty! ");
    }

    sm_lts.iter().for_each(|lt| {
        let triples = Triples {
            sm,
            poms: poms.iter().map(|pom| pom.into()).collect(),
        };

        let gms = sm.graph_maps.iter().collect();

        let quads = Quads { triples, gms };

        update_lt_map(&mut result, lt, quads);
    });

    for pom in poms {
        let oms = &pom.object_maps;
        let pms = &pom.predicate_maps;

        let pom_gms = pom.graph_maps.iter();
        for pm in pms {
            pm.tm_info.logical_targets.iter().for_each(|lt| {
                let ref_pom = RefPOM {
                    pm: vec![pm],
                    om: oms.iter().collect(),
                };
                let pm_gms = pm.graph_maps.iter();
                let gms = pm_gms.chain(pom_gms.clone()).collect();

                let triples = Triples {
                    sm,
                    poms: vec![ref_pom],
                };

                let quads = Quads { triples, gms };

                update_lt_map(&mut result, lt, quads);
            });
        }

        for om in oms {
            om.tm_info.logical_targets.iter().for_each(|lt| {
                let ref_pom = RefPOM {
                    pm: pms.iter().collect(),
                    om: vec![om],
                };

                let triples = Triples {
                    sm,
                    poms: vec![ref_pom],
                };
                let om_gms = om.graph_maps.iter();
                let gms = om_gms.chain(pom_gms.clone()).collect();

                let quads = Quads { triples, gms };

                update_lt_map(&mut result, lt, quads);
            })
        }
    }

    result
}

fn update_lt_map<'a>(
    result: &mut HashMap<String, Vec<Quads<'a>>>,
    lt: &LogicalTarget,
    quads: Quads<'a>,
) {
    if result.get(&lt.identifier).is_some() {
        let inserted_quads = result.get_mut(&lt.identifier).unwrap();
        if !inserted_quads.contains(&quads) {
            inserted_quads.push(quads);
        }
    } else {
        result.insert(lt.identifier.clone(), vec![quads]);
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
        let tm_prefix = format!("?tm{}", tm_idx);
        result_map.insert(triples_map.identifier.clone(), tm_prefix.clone());

        let subject_map = &triples_map.subject_map;
        let sm_gm_variables =
            subject_map.graph_maps.iter().enumerate().map(|(idx, gm)| {
                (
                    gm.tm_info.identifier.clone(),
                    format!("{}_sm_gm{}", tm_prefix, idx),
                )
            });
        result_map.extend(sm_gm_variables);

        result_map.insert(
            subject_map.tm_info.identifier.clone(),
            format!("{}_sm", tm_prefix),
        );

        for (pom_idx, pom) in triples_map.po_maps.iter().enumerate() {
            let pom_gm_variables =
                pom.graph_maps.iter().enumerate().map(|(idx, gm)| {
                    (
                        gm.tm_info.identifier.clone(),
                        format!("{}_pom{}_gm{}", tm_prefix, pom_idx, idx),
                    )
                });
            result_map.extend(pom_gm_variables);

            for (pm_idx, pm) in pom.predicate_maps.iter().enumerate() {
                let pm_gm_variables =
                    pm.graph_maps.iter().enumerate().map(|(idx, gm)| {
                        (
                            gm.tm_info.identifier.clone(),
                            format!(
                                "{}_p{}_{}_gm{}",
                                tm_prefix, pom_idx, pm_idx, idx
                            ),
                        )
                    });
                result_map.extend(pm_gm_variables);

                result_map.insert(
                    pm.tm_info.identifier.clone(),
                    format!("{}_p{}_{}", tm_prefix, pom_idx, pm_idx),
                );
            }

            for (om_idx, om) in pom.object_maps.iter().enumerate() {
                let om_gm_variables =
                    om.graph_maps.iter().enumerate().map(|(idx, gm)| {
                        (
                            gm.tm_info.identifier.clone(),
                            format!(
                                "{}_p{}_{}_gm{}",
                                tm_prefix, pom_idx, om_idx, idx
                            ),
                        )
                    });
                result_map.extend(om_gm_variables);
                result_map.insert(
                    om.tm_info.identifier.clone(),
                    format!("{}_o{}_{}", tm_prefix, pom_idx, om_idx),
                );
            }
        }
    }

    result_map
}
