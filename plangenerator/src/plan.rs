use std::cell::RefCell;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::marker::PhantomData;
use std::path::PathBuf;
use std::rc::Rc;

use anyhow::Result;
use operator::display::PrettyDisplay;
use operator::{Join, Operator, Serializer, Source, Target};
use petgraph::dot::{Config, Dot};
use petgraph::graph::{DiGraph, NodeIndex};
use serde_json::json;

use crate::error::PlanError;

type DiGraphOperators = DiGraph<PlanNode, PlanEdge>;
pub type RcRefCellDiGraph = Rc<RefCell<DiGraphOperators>>;

type VSourceIdxs = Vec<NodeIndex>;
pub type RcRefCellVSourceIdxs = Rc<RefCell<VSourceIdxs>>;

// Plan states in unit structs

#[derive(Debug, Clone)]
pub struct Init {}
#[derive(Debug, Clone)]
pub struct Processed {}
#[derive(Debug, Clone)]
pub struct Serialized {}
#[derive(Debug, Clone)]
pub struct Sunk {}

#[derive(Debug, Clone)]
pub struct Plan<T> {
    _t:            PhantomData<T>,
    pub graph:     RcRefCellDiGraph,
    pub sources:   RcRefCellVSourceIdxs,
    pub last_node: Option<NodeIndex>,
}

impl Plan<()> {
    pub fn new() -> Plan<Init> {
        Plan {
            _t:        PhantomData,
            graph:     Rc::new(RefCell::new(DiGraph::new())),
            sources:   Rc::new(RefCell::new(Vec::new())),
            last_node: None,
        }
    }
}

impl<T> Plan<T> {
    fn empty_plan_apply_check(&self) -> Result<(), PlanError> {
        if self.graph.borrow().node_count() == 0 {
            return Err(PlanError::EmptyPlan);
        }
        Ok(())
    }

    pub fn next_idx<O>(&self, idx: Option<NodeIndex>) -> Plan<O> {
        Plan {
            _t:        PhantomData,
            graph:     Rc::clone(&self.graph),
            sources:   Rc::clone(&self.sources),
            last_node: idx,
        }
    }

    pub fn write_fmt(
        &mut self,
        path: PathBuf,
        fmt: &dyn Fn(Dot<&DiGraphOperators>) -> String,
    ) -> Result<()> {
        let graph = &*self.graph.borrow_mut();
        let dot_string = fmt(Dot::with_config(graph, &[Config::EdgeNoLabel]));
        write_string_to_file(path, dot_string)?;
        Ok(())
    }

    pub fn write_pretty(&mut self, path: PathBuf) -> Result<()> {
        self.write_fmt(path, &|dot| format!("{}", dot))?;
        Ok(())
    }

    pub fn write(&mut self, path: PathBuf) -> Result<()> {
        self.write_fmt(path, &|dot| format!("{:?}", dot))?;
        Ok(())
    }
}

fn write_string_to_file(
    path: PathBuf,
    content: String,
) -> Result<(), anyhow::Error> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    write!(writer, "{}", content)?;
    Ok(())
}

impl Plan<Init> {
    pub fn source(&mut self, source: Source) -> Plan<Processed> {
        let graph = &mut *self.graph.borrow_mut();
        let source_op = Operator::SourceOp {
            config: source.clone(),
        };
        let sources = &mut *self.sources.borrow_mut();

        let plan_node = PlanNode {
            id:       format!("Source_{}", graph.node_count()),
            operator: source_op,
        };
        let idx = Some(graph.add_node(plan_node));
        sources.push(idx.unwrap());
        self.next_idx(idx)
    }
}

impl Plan<Processed> {
    pub fn join(
        &mut self,
        other_op: &Plan<Processed>,
    ) -> Result<NotAliasedJoinedPlan<Processed>, PlanError> {
        let left_node = self.last_node.unwrap();
        let right_node = other_op.last_node.unwrap();

        Ok(NotAliasedJoinedPlan {
            plan: self.clone(),
            left_node,
            right_node,
        })
    }

    pub fn apply(
        &mut self,
        operator: &Operator,
        node_id_prefix: &str,
    ) -> Result<Plan<Processed>, PlanError> {
        self.empty_plan_apply_check()?;
        let prev_node_idx = self
            .last_node
            .ok_or(PlanError::DanglingApplyOperator(operator.clone()))?;

        match operator {
            Operator::SourceOp { .. }
            | Operator::TargetOp { .. }
            | Operator::SerializerOp { .. } => {
                return Err(PlanError::WrongApplyOperator(operator.clone()))
            }
            _ => (),
        };

        let graph = &mut *self.graph.borrow_mut();
        let id_num = graph.node_count();

        let plan_node = PlanNode {
            id:       format!("{}_{}", node_id_prefix, id_num),
            operator: operator.clone(),
        };

        let new_node_idx = graph.add_node(plan_node);

        let plan_edge = PlanEdge {
            key:   std::any::type_name::<()>().to_string(),
            value: "MappingTuple".to_string(),
        };

        graph.add_edge(prev_node_idx, new_node_idx, plan_edge);

        Ok(self.next_idx(Some(new_node_idx)))
    }

    pub fn serialize(
        &mut self,
        serializer: Serializer,
    ) -> Result<Plan<Serialized>, PlanError> {
        self.empty_plan_apply_check()?;
        let prev_node_idx = self.last_node.ok_or(
            PlanError::DanglingApplyOperator(Operator::SerializerOp {
                config: serializer.clone(),
            }),
        )?;

        let graph = &mut *self.graph.borrow_mut();
        let plan_node = PlanNode {
            id:       format!("Serialize_{}", graph.node_count()),
            operator: Operator::SerializerOp { config: serializer },
        };

        let node_idx = graph.add_node(plan_node);

        let plan_edge = PlanEdge {
            key:   std::any::type_name::<()>().to_string(),
            value: "MappingTuple".to_string(),
        };

        graph.add_edge(prev_node_idx, node_idx, plan_edge);
        Ok(self.next_idx(Some(node_idx)))
    }
}

#[derive(Debug, Clone)]
pub struct NotAliasedJoinedPlan<T> {
    plan:       Plan<T>,
    left_node:  NodeIndex,
    right_node: NodeIndex,
}

impl NotAliasedJoinedPlan<Processed> {
    pub fn alias(
        &mut self,
        alias: &str,
    ) -> Result<AliasedJoinedPlan<Processed>, PlanError> {
        Ok(AliasedJoinedPlan {
            plan:       self.plan.clone(),
            alias:      alias.to_string(),
            left_node:  self.left_node,
            right_node: self.right_node,
        })
    }
}

#[derive(Debug, Clone)]
pub struct AliasedJoinedPlan<T> {
    plan:       Plan<T>,
    alias:      String,
    left_node:  NodeIndex,
    right_node: NodeIndex,
}

impl AliasedJoinedPlan<Processed> {
    pub fn where_by<A>(
        &mut self,
        attributes: Vec<A>,
    ) -> Result<WhereByPlan<Processed>, PlanError>
    where
        A: Into<String>,
    {
        let left_attributes: Vec<String> =
            attributes.into_iter().map(|a| a.into()).collect();
        Ok(WhereByPlan {
            joined_plan: self.clone(),
            left_attributes,
        })
    }

    pub fn cross(&mut self) -> Result<Plan<Processed>, PlanError> {
        let graph = &mut *self.plan.graph.borrow_mut();

        let join_config = Join {
            join_type: operator::JoinType::CrossJoin,
            join_alias: self.alias.clone(),
            ..Default::default()
        };

        let join_node = PlanNode {
            id:       format!(
                "Join_{:?}_{:?}",
                self.left_node, self.right_node
            ),
            operator: Operator::JoinOp {
                config: join_config,
            },
        };

        // TODO: Cross join node to the plan <01-09-23, yourname> //
        let node_idx = graph.add_node(join_node);

        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct WhereByPlan<T> {
    joined_plan:     AliasedJoinedPlan<T>,
    left_attributes: Vec<String>,
}

impl WhereByPlan<Processed> {
    // TODO: Allow choice in join type and predicate type <30-08-23, Min Oo> //
    pub fn compared_to<A>(
        &mut self,
        attributes: Vec<A>,
    ) -> Result<Plan<Processed>, PlanError>
    where
        A: Into<String>,
    {
        let joined_plan = &self.joined_plan;
        let plan = &joined_plan.plan;
        let graph = &mut *plan.graph.borrow_mut();

        let left_attributes = self.left_attributes.to_owned();
        let right_attributes: Vec<String> =
            attributes.into_iter().map(|a| a.into()).collect();

        let left_right_attr_pairs: Vec<(String, String)> = left_attributes
            .clone()
            .into_iter()
            .zip(right_attributes.clone().into_iter())
            .collect();

        let join_op = Operator::JoinOp {
            config: Join {
                join_alias: joined_plan.alias.clone(),
                left_right_attr_pairs,
                join_type: operator::JoinType::InnerJoin,
                predicate_type: operator::PredicateType::Equal,
            },
        };

        let join_node = PlanNode {
            id:       format!("Join_{}", graph.node_count()),
            operator: join_op,
        };

        let node_idx = graph.add_node(join_node);

        let left_node = joined_plan.left_node;
        let left_edge = PlanEdge {
            key:   format!("{:?}", left_attributes),
            value: "MappingTuple".to_string(),
        };

        graph.add_edge(left_node, node_idx, left_edge);

        let right_node = joined_plan.right_node;
        let right_edge = PlanEdge {
            key:   format!("{:?}", right_attributes),
            value: "MappingTuple".to_string(),
        };

        graph.add_edge(right_node, node_idx, right_edge);

        Ok(plan.next_idx(Some(node_idx)))
    }
}

impl Plan<Serialized> {
    pub fn sink(&mut self, sink: Target) -> Result<Plan<Sunk>, PlanError> {
        if self.last_node.is_none() {
            return Err(PlanError::EmptyPlan);
        }

        let graph = &mut *self.graph.borrow_mut();
        let plan_node = PlanNode {
            id:       format!("Sink_{}", graph.node_count()),
            operator: Operator::TargetOp { config: sink },
        };

        let node_idx = graph.add_node(plan_node);
        let prev_node_idx = self.last_node.unwrap();

        let plan_edge = PlanEdge {
            key:   std::any::type_name::<()>().to_string(),
            value: "Serialized Format".to_string(),
        };
        graph.add_edge(prev_node_idx, node_idx, plan_edge);

        Ok(self.next_idx(Some(node_idx)))
    }
}

#[derive(Debug, Clone)]
pub struct PlanEdge {
    pub key:   String,
    pub value: String,
}

impl Display for PlanEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.key, "->", self.value)
    }
}

#[derive(Clone, Hash)]
pub struct PlanNode {
    pub id:       String,
    pub operator: Operator,
}

impl Debug for PlanNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = json!({"id": self.id, "operator": self.operator});
        f.write_str(&serde_json::to_string(&json).unwrap())
    }
}

impl PrettyDisplay for PlanNode {
    fn pretty_string(&self) -> Result<String> {
        let content = self.operator.pretty_string()?;

        Ok(format!("Id: {}\n{}", self.id, content))
    }
}

impl Display for PlanNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id:{} \n{}",
            self.id,
            self.operator.pretty_string().unwrap()
        )
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use operator::{Projection, Rename};

    use super::*;

    #[test]
    fn test_plan_source() {
        let mut plan = Plan::new();
        let source = Source {
            config:              HashMap::new(),
            source_type:         operator::IOType::File,
            reference_iterators: vec![],
            data_format:         operator::formats::DataFormat::CSV,
        };
        plan.source(source.clone());
        let graph = plan.graph.borrow();

        assert!(graph.node_count() == 1);
        assert!(graph.edge_count() == 0);
        let retrieved_node = graph.node_weights().next();

        assert!(retrieved_node.is_some());
        let source_op = Operator::SourceOp { config: source };
        assert!(retrieved_node.unwrap().operator == source_op);
    }

    #[test]
    fn test_plan_apply() -> std::result::Result<(), PlanError> {
        let mut plan = Plan::new();
        let source = Source {
            config:              HashMap::new(),
            source_type:         operator::IOType::File,
            reference_iterators: vec![],
            data_format:         operator::formats::DataFormat::CSV,
        };

        let project_op = Operator::ProjectOp {
            config: Projection {
                projection_attributes: HashSet::new(),
            },
        };
        let rename_op = Operator::RenameOp {
            config: Rename {
                rename_pairs: HashMap::from([(
                    "first".to_string(),
                    "last".to_string(),
                )]),
            },
        };

        let _ = plan
            .source(source.clone())
            .apply(&project_op, "Projection")?
            .apply(&rename_op, "Rename")?;

        let graph = plan.graph.borrow();

        assert!(
            graph.node_count() == 3,
            "Number of nodes should be 3 but it is instead: {}",
            graph.node_count()
        );
        assert!(
            graph.edge_count() == 2,
            "Number of edges should be 2 but it is instead: {}",
            graph.edge_count()
        );

        Ok(())
    }
}