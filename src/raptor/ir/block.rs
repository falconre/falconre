use crate::raptor::ir;
use pyo3::class::PyObjectProtocol;
use pyo3::prelude::*;

#[pyclass]
#[derive(Eq, Hash, PartialEq)]
pub struct Block {
    pub(crate) block: raptor::ir::Block<raptor::ir::Constant>,
}

#[pymethods]
impl Block {
    #[getter(index)]
    fn index(&self) -> usize {
        self.block.index()
    }

    fn instruction(&self, index: usize) -> Option<ir::Instruction> {
        self.block
            .instruction(index)
            .map(|instruction| instruction.clone().into())
    }

    #[getter(instructions)]
    fn instructions(&self) -> Vec<ir::Instruction> {
        self.block
            .instructions()
            .iter()
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

    fn __hash__(&self) -> PyResult<isize> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        Ok(s.finish() as isize)
    }
}

impl From<raptor::ir::Block<raptor::ir::Constant>> for Block {
    fn from(block: raptor::ir::Block<raptor::ir::Constant>) -> Block {
        Block { block }
    }
}
