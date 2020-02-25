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

    fn address(&self) -> u64 {
        self.function.address()
    }

    fn index(&self) -> Option<usize> {
        self.function.index()
    }

    fn name(&self) -> &str {
        self.function.name()
    }

    fn blocks(&self) -> Vec<ir::Block> {
        self.function
            .blocks()
            .into_iter()
            .map(|block| block.clone().into())
            .collect()
    }
}

impl From<raptor::ir::Function<raptor::ir::Constant>> for Function {
    fn from(function: raptor::ir::Function<raptor::ir::Constant>) -> Function {
        Function { function }
    }
}
