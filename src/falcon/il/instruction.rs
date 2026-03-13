use crate::map_err;
use pyo3::prelude::*;

use super::Operation;

#[pyclass]
pub struct Instruction {
    pub(crate) instruction: falcon::il::Instruction,
}

#[pymethods]
impl Instruction {
    fn operation(&self) -> Operation {
        self.instruction.operation().clone().into()
    }

    fn index(&self) -> usize {
        self.instruction.index()
    }

    fn comment(&self) -> Option<String> {
        self.instruction.comment().map(|s| s.to_string())
    }

    fn address(&self) -> Option<u64> {
        self.instruction.address()
    }

    fn json(&self) -> PyResult<String> {
        map_err(serde_json::to_string(&self.instruction))
    }

    fn __str__(&self) -> String {
        self.instruction.to_string()
    }

    fn __repr__(&self) -> String {
        self.instruction.to_string()
    }
}

impl From<falcon::il::Instruction> for Instruction {
    fn from(instruction: falcon::il::Instruction) -> Instruction {
        Instruction { instruction }
    }
}
