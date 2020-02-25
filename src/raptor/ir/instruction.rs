use crate::raptor::ir;
use pyo3::class::PyObjectProtocol;
use pyo3::prelude::*;

#[pyclass]
pub struct Instruction {
    pub(crate) instruction: raptor::ir::Instruction<raptor::ir::Constant>,
}

#[pymethods]
impl Instruction {
    fn index(&self) -> usize {
        self.instruction.index()
    }

    fn operation(&self) -> ir::Operation {
        self.instruction.operation().clone().into()
    }

    fn address(&self) -> Option<u64> {
        self.instruction.address()
    }
}

#[pyproto]
impl<'p> PyObjectProtocol<'p> for Instruction {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.instruction.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.instruction.to_string())
    }
}

impl From<raptor::ir::Instruction<raptor::ir::Constant>> for Instruction {
    fn from(instruction: raptor::ir::Instruction<raptor::ir::Constant>) -> Instruction {
        Instruction { instruction }
    }
}
