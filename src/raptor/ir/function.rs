use crate::map_err;
use crate::raptor::ir;
use pyo3::prelude::*;

#[pyclass]
pub struct Function {
    pub(crate) function: raptor::ir::Function<raptor::ir::Constant>,
}

#[pymethods]
impl Function {
    #[staticmethod]
    fn from_il(function: &crate::falcon::il::Function) -> PyResult<Function> {
        map_err(raptor::ir::Function::<raptor::ir::Constant>::from_il(
            &function.function,
        ))
        .map(|function| function.into())
    }

    #[getter(address)]
    fn address(&self) -> u64 {
        self.function.address()
    }

    #[getter(index)]
    fn index(&self) -> Option<usize> {
        self.function.index()
    }

    #[getter(control_flow_graph)]
    fn control_flow_graph(&self) -> ir::ControlFlowGraph {
        ir::ControlFlowGraph {
            control_flow_graph: self.function.control_flow_graph().clone(),
        }
    }

    #[getter(name)]
    fn name(&self) -> &str {
        self.function.name()
    }

    #[getter(blocks)]
    fn blocks(&self) -> Vec<ir::Block> {
        self.function
            .blocks()
            .into_iter()
            .map(|block| block.clone().into())
            .collect()
    }

    fn block(&self, index: usize) -> PyResult<ir::Block> {
        map_err(self.function.block(index)).map(|block| block.clone().into())
    }

    #[getter(edges)]
    fn edges(&self) -> Vec<ir::Edge> {
        self.function
            .edges()
            .into_iter()
            .map(|edge| edge.clone().into())
            .collect()
    }
}

impl From<raptor::ir::Function<raptor::ir::Constant>> for Function {
    fn from(function: raptor::ir::Function<raptor::ir::Constant>) -> Function {
        Function { function }
    }
}
