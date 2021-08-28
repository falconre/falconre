use crate::map_err;
use pyo3::class::PyObjectProtocol;
use pyo3::prelude::*;

use super::Instruction;

#[pyclass]
pub struct Block {
    pub(crate) block: falcon::il::Block,
}

#[pymethods]
impl Block {
    fn address(&self) -> Option<u64> {
        self.block.address()
    }

    fn index(&self) -> usize {
        self.block.index()
    }

    fn instructions(&self) -> Vec<Instruction> {
        self.block
            .instructions()
            .iter()
            .map(|i| i.clone().into())
            .collect::<Vec<Instruction>>()
    }

    fn json(&self) -> PyResult<String> {
        map_err(serde_json::to_string(&self.block))
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

impl From<falcon::il::Block> for Block {
    fn from(block: falcon::il::Block) -> Block {
        Block { block }
    }
}
