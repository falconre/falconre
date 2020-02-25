use crate::map_err;
use pyo3::prelude::*;

use super::{Block, ControlFlowGraph};

#[pyclass]
pub struct Function {
    pub(crate) function: falcon::il::Function,
}

#[pymethods]
impl Function {
    fn address(&self) -> u64 {
        self.function.address()
    }

    fn control_flow_graph(&self) -> ControlFlowGraph {
        self.function.control_flow_graph().clone().into()
    }

    fn name(&self) -> String {
        self.function.name()
    }

    fn index(&self) -> Option<usize> {
        self.function.index()
    }

    fn blocks(&self) -> Vec<Block> {
        self.control_flow_graph().blocks()
    }

    fn json(&self) -> PyResult<String> {
        map_err(serde_json::to_string(&self.function))
    }
}

impl From<falcon::il::Function> for Function {
    fn from(function: falcon::il::Function) -> Function {
        Function { function }
    }
}
