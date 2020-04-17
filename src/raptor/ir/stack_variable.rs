use crate::map_err;
use pyo3::class::basic::CompareOp;
use pyo3::class::PyObjectProtocol;
use pyo3::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[pyclass]
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct StackVariable {
    pub(crate) stack_variable: raptor::ir::StackVariable,
}

#[pymethods]
impl StackVariable {
    #[new]
    fn new(offset: isize, bits: usize) -> StackVariable {
        StackVariable {
            stack_variable: raptor::ir::StackVariable::new(offset, bits),
        }
    }

    fn offset(&self) -> isize {
        self.stack_variable.offset()
    }

    fn bits(&self) -> usize {
        self.stack_variable.bits()
    }

    fn json(&self) -> PyResult<String> {
        map_err(serde_json::to_string(&self.stack_variable))
    }
}

#[pyproto]
impl<'p> PyObjectProtocol<'p> for StackVariable {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.stack_variable.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.stack_variable.to_string())
    }

    fn __hash__(&self) -> PyResult<isize> {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        Ok(s.finish() as isize)
    }

    fn __richcmp__(&self, other: StackVariable, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Eq => Ok(*self == other),
            _ => Ok(false),
        }
    }
}

impl From<raptor::ir::StackVariable> for StackVariable {
    fn from(stack_variable: raptor::ir::StackVariable) -> StackVariable {
        StackVariable { stack_variable }
    }
}
