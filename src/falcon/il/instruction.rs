use crate::map_err;
use pyo3::class::PyObjectProtocol;
use pyo3::prelude::*;

use super::Operation;

#[pyclass]
pub struct Instruction {
    pub(crate) instruction: falcon::il::Instruction,
}

#[pymethods]
impl Instruction {
    #[getter(operation)]
    fn operation(&self) -> Operation {
        self.instruction.operation().clone().into()
    }

    #[getter(instruction)]
    fn index(&self) -> usize {
        self.instruction.index()
    }

    #[getter(comment)]
    fn comment(&self) -> Option<String> {
        self.instruction.comment().map(|s| s.to_string())
    }

    #[getter(address)]
    fn address(&self) -> Option<u64> {
        self.instruction.address()
    }

    #[getter(json)]
    fn json(&self) -> PyResult<String> {
        map_err(serde_json::to_string(&self.instruction))
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

impl From<falcon::il::Instruction> for Instruction {
    fn from(instruction: falcon::il::Instruction) -> Instruction {
        Instruction { instruction }
    }
}
