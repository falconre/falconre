use crate::map_err;
use crate::raptor::ir;
use pyo3::class::PyObjectProtocol;
use pyo3::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[pyclass]
#[derive(Eq, Hash, PartialEq)]
pub struct LValue {
    pub(crate) lvalue: raptor::ir::LValue<raptor::ir::Constant>,
}

#[pymethods]
impl LValue {
    fn variable(&self) -> Option<ir::Variable> {
        self.lvalue
            .variable()
            .map(|variable| variable.clone().into())
    }

    fn scalar(&self) -> Option<ir::Scalar> {
        self.lvalue.scalar().map(|scalar| scalar.clone().into())
    }

    fn stack_variable(&self) -> Option<ir::StackVariable> {
        self.lvalue
            .stack_variable()
            .map(|stack_variable| stack_variable.clone().into())
    }

    fn derefernce(&self) -> Option<ir::Dereference> {
        self.lvalue
            .dereference()
            .map(|dereference| dereference.clone().into())
    }

    fn bits(&self) -> usize {
        self.lvalue.bits()
    }

    fn json(&self) -> PyResult<String> {
        map_err(serde_json::to_string(&self.lvalue))
    }
}

#[pyproto]
impl<'p> PyObjectProtocol<'p> for LValue {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.lvalue.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.lvalue.to_string())
    }

    fn __hash__(&self) -> PyResult<isize> {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        Ok(s.finish() as isize)
    }
}

impl From<raptor::ir::LValue<raptor::ir::Constant>> for LValue {
    fn from(lvalue: raptor::ir::LValue<raptor::ir::Constant>) -> LValue {
        LValue { lvalue }
    }
}
