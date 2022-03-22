use crate::map_err;
use pyo3::prelude::*;

use super::{Block, ControlFlowGraph};

#[pyclass]
pub struct Function {
    pub(crate) function: falcon::il::Function,
}

#[pymethods]
impl Function {
    #[getter(address)]
    fn address(&self) -> u64 {
        self.function.address()
    }

    #[getter(control_flow_graph)]
    fn control_flow_graph(&self) -> ControlFlowGraph {
        self.function.control_flow_graph().clone().into()
    }

    #[getter(name)]
    fn name(&self) -> String {
        self.function.name()
    }

    #[getter(index)]
    fn index(&self) -> Option<usize> {
        self.function.index()
    }

    #[getter(blocks)]
    fn blocks(&self) -> Vec<Block> {
        self.control_flow_graph().blocks()
    }

    #[getter(json)]
    fn json(&self) -> PyResult<String> {
        map_err(serde_json::to_string(&self.function))
    }
}

impl From<falcon::il::Function> for Function {
    fn from(function: falcon::il::Function) -> Function {
        Function { function }
    }
}
