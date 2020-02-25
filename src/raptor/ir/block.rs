use crate::raptor::ir;
use pyo3::class::PyObjectProtocol;
use pyo3::prelude::*;

#[pyclass]
pub struct Block {
    pub(crate) block: raptor::ir::Block<raptor::ir::Constant>,
}

#[pymethods]
impl Block {
    fn index(&self) -> usize {
        self.block.index()
    }

    fn instructions(&self) -> Vec<ir::Instruction> {
        self.block
            .instructions()
            .into_iter()
            .map(|instruction| instruction.clone().into())
            .collect()
    }
}

#[pyproto]
impl<'p> PyObjectProtocol<'p> for Block {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.block.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.block.to_string())
    }
}

impl From<raptor::ir::Block<raptor::ir::Constant>> for Block {
    fn from(block: raptor::ir::Block<raptor::ir::Constant>) -> Block {
        Block { block }
    }
}
