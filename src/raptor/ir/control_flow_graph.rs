use crate::raptor::ir;
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct ControlFlowGraph {
    pub(crate) control_flow_graph: raptor::ir::ControlFlowGraph<raptor::ir::Constant>,
}

#[pymethods]
impl ControlFlowGraph {
    fn entry(&self) -> Option<usize> {
        self.control_flow_graph.entry()
    }

    fn blocks(&self) -> Vec<ir::Block> {
        self.control_flow_graph
            .blocks()
            .into_iter()
            .map(|block| block.clone().into())
            .collect()
    }

    fn edges(&self) -> Vec<ir::Edge> {
        self.control_flow_graph
            .edges()
            .into_iter()
            .map(|edge| edge.clone().into())
            .collect()
    }
}

impl From<raptor::ir::ControlFlowGraph<raptor::ir::Constant>> for ControlFlowGraph {
    fn from(
        control_flow_graph: raptor::ir::ControlFlowGraph<raptor::ir::Constant>,
    ) -> ControlFlowGraph {
        ControlFlowGraph { control_flow_graph }
    }
}
