use std::path::PathBuf;

use anyhow::Result;
use operator::plan::{Plan, Sunk};
use petgraph::Direction;
use tokio::runtime::Runtime;

pub struct Executor {
    rt: Runtime,
}

impl Executor {
    pub fn create() -> Result<Executor> {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .thread_name("Meamer-mapper")
            .build()?;

        Ok(Executor { rt })
    }

    // TODO: consider join operations in the plan <17-08-23, Sitt Min Oo> //
    pub fn run(&mut self, plan: Plan<Sunk>) {
        let source_idxs = plan.sources.borrow();
        let graph = plan.graph.borrow();

        let rt = &self.rt;

        source_idxs.iter().for_each(|source_id| {
            let source = graph.node_weight(*source_id);
            let operators =
                graph.neighbors_directed(*source_id, Direction::Outgoing);
        });

        todo!()
    }
}
