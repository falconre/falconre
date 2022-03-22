use pyo3::prelude::*;

use super::{Block, Edge};

#[pyclass]
pub struct ControlFlowGraph {
    control_flow_graph: falcon::il::ControlFlowGraph,
}

#[pymethods]
impl ControlFlowGraph {
    #[getter(entry)]
    fn entry(&self) -> Option<usize> {
        self.control_flow_graph.entry()
    }

    #[getter(exit)]
    fn exit(&self) -> Option<usize> {
        self.control_flow_graph.exit()
    }

    pub fn block(&self, index: usize) -> Option<Block> {
        self.control_flow_graph
            .block(index)
            .map(|b| b.clone().into())
            .ok()
    }

    #[getter(blocks)]
    pub fn blocks(&self) -> Vec<Block> {
        self.control_flow_graph
            .blocks()
            .into_iter()
            .map(|b| b.clone().into())
            .collect()
    }

    fn edge(&self, head: usize, tail: usize) -> Option<Edge> {
        self.control_flow_graph
            .edge(head, tail)
            .map(|e| e.clone().into())
            .ok()
    }

    #[getter(edges)]
    fn edges(&self) -> Vec<Edge> {
        self.control_flow_graph
            .edges()
            .into_iter()
            .map(|e| e.clone().into())
            .collect()
    }

    fn edges_in(&self, index: usize) -> Option<Vec<Edge>> {
        self.control_flow_graph
            .edges_in(index)
            .ok()
            .map(|edges| edges.into_iter().map(|e| e.clone().into()).collect())
    }

    fn edges_out(&self, index: usize) -> Option<Vec<Edge>> {
        self.control_flow_graph
            .edges_out(index)
            .ok()
            .map(|edges| edges.into_iter().map(|e| e.clone().into()).collect())
    }

    #[getter(dot_graph)]
    fn dot_graph(&self) -> String {
        self.control_flow_graph.graph().dot_graph()
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.control_flow_graph.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.control_flow_graph.to_string())
    }
}

impl From<falcon::il::ControlFlowGraph> for ControlFlowGraph {
    fn from(control_flow_graph: falcon::il::ControlFlowGraph) -> ControlFlowGraph {
        ControlFlowGraph { control_flow_graph }
    }
}
