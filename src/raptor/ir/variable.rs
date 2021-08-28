use crate::map_err;
use crate::raptor::ir;
use pyo3::class::basic::CompareOp;
use pyo3::class::PyObjectProtocol;
use pyo3::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[pyclass]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Variable {
    pub(crate) variable: raptor::ir::Variable,
}

#[pymethods]
impl Variable {
    #[getter(scalar)]
    fn scalar(&self) -> Option<ir::Scalar> {
        self.variable.scalar().map(|scalar| scalar.clone().into())
    }

    #[getter(stack_variable)]
    fn stack_variable(&self) -> Option<ir::StackVariable> {
        self.variable
            .stack_variable()
            .map(|stack_variable| stack_variable.clone().into())
    }

    #[getter(bits)]
    fn bits(&self) -> usize {
        self.variable.bits()
    }

    #[getter(json)]
    fn json(&self) -> PyResult<String> {
        map_err(serde_json::to_string(&self.variable))
    }

    #[getter(e)]
    fn e(&self) -> ir::Expression {
        ir::Expression {
            expression: self.variable.clone().into(),
        }
    }
}

#[pyproto]
impl<'p> PyObjectProtocol<'p> for Variable {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.variable.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.variable.to_string())
    }

    fn __hash__(&self) -> PyResult<isize> {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        Ok(s.finish() as isize)
    }

    fn __richcmp__(&self, other: Variable, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Eq => Ok(*self == other),
            _ => Ok(false),
        }
    }
}

impl From<raptor::ir::Variable> for Variable {
    fn from(variable: raptor::ir::Variable) -> Variable {
        Variable { variable }
    }
}
