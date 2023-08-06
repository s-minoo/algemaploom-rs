use std::cell::RefCell;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::marker::PhantomData;
use std::path::PathBuf;
use std::rc::Rc;

use anyhow::Result;
use petgraph::dot::{Config, Dot};
use petgraph::graph::{DiGraph, NodeIndex};

use crate::tuples::MappingTuple;
use crate::{Operator, Serializer, Source, Target};

type DiGraphOperators = DiGraph<PlanNode, PlanEdge>;
pub type RcRefCellDiGraph = Rc<RefCell<DiGraphOperators>>;

#[derive(Debug, Clone)]
pub struct Plan<T> {
    _t:            PhantomData<T>,
    pub graph:     RcRefCellDiGraph,
    pub last_node: Option<NodeIndex>,
}

#[derive(thiserror::Error, Debug)]
pub enum PlanError {
    #[error("Invalid to add non-leaf operator to an empty plan")]
    EmptyPlan,

    #[error("The operator {0} is not supported")]
    OperatorNotSupported(&'static str),

    #[error(
        "The given operator cannot be added using the apply function:\n{0:?}"
    )]
    WrongApplyOperator(Operator),

    #[error("The given operator needs to be connected to a previous opeartor: \n{0:?}")]
    DanglingApplyOperator(Operator),

    #[error("Something else happened: {0:?}")]
    AuxError(String),
}

impl<T> Plan<T> {
    fn empty_plan_apply_check(&self) -> Result<(), PlanError> {
        if self.graph.borrow().node_count() == 0 {
            return Err(PlanError::EmptyPlan);
        }
        Ok(())
    }

    pub fn new() -> Plan<()> {
        Plan {
            _t:        PhantomData,
            graph:     Rc::new(RefCell::new(DiGraph::new())),
            last_node: None,
        }
    }

    pub fn source(&mut self, source: Source) -> Plan<MappingTuple> {
        let graph = &mut *self.graph.borrow_mut();
        let source_op = Operator::SourceOp { config: source };
        let plan_node = PlanNode {
            id:       format!("Source_{}", graph.node_count()),
            operator: source_op,
        };
        let idx = Some(graph.add_node(plan_node));

        Plan {
            _t:        PhantomData,
            graph:     Rc::clone(&self.graph),
            last_node: idx,
        }
    }

    pub fn write(&mut self, path: PathBuf) -> Result<()> {
        let graph = &*self.graph.borrow_mut();
        let dot_string =
            format!("{}", Dot::with_config(graph, &[Config::EdgeNoLabel]));

        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        write!(writer, "{}", dot_string)?;
        Ok(())
    }
}

impl<MappingTuple> Plan<MappingTuple> {
    pub fn apply(
        &mut self,
        operator: &Operator,
        node_id_prefix: &str,
    ) -> Result<Plan<MappingTuple>, PlanError> {
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

        Ok(Plan {
            _t:        PhantomData,
            graph:     Rc::clone(&self.graph),
            last_node: Some(new_node_idx),
        })
    }

    pub fn serialize(
        &mut self,
        serializer: Serializer,
    ) -> Result<SerializedPlan, PlanError> {
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
        Ok(SerializedPlan {
            graph:     Rc::clone(&self.graph),
            last_node: Some(node_idx),
        })
    }
}

#[derive(Debug, Clone)]
pub struct SerializedPlan {
    pub graph:     RcRefCellDiGraph,
    pub last_node: Option<NodeIndex>,
}

impl SerializedPlan {
    pub fn sink(&mut self, sink: Target) -> Result<(), PlanError> {
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

        Ok(())
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

#[derive(Debug, Clone, Hash)]
pub struct PlanNode {
    pub id:       String,
    pub operator: Operator,
}

impl Display for PlanNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id:{} \n{}",
            self.id,
            serde_json::to_string_pretty(&self.operator).unwrap()
        )
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use super::*;
    use crate::{Projection, Rename};

    #[test]
    fn test_plan_source() {
        let mut plan = Plan::<MappingTuple>::new();
        let source = Source {
            config:      HashMap::new(),
            source_type: crate::IOType::File,
            data_format: crate::formats::DataFormat::CSV,
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
        let mut plan = Plan::<MappingTuple>::new();
        let source = Source {
            config:      HashMap::new(),
            source_type: crate::IOType::File,
            data_format: crate::formats::DataFormat::CSV,
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
