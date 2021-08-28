use crate::map_err;
use crate::raptor::ir;
use pyo3::class::PyObjectProtocol;
use pyo3::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[pyclass]
#[derive(Eq, Hash, PartialEq)]
pub struct RValue {
    pub(crate) rvalue: raptor::ir::RValue<raptor::ir::Constant>,
}

#[pymethods]
impl RValue {
    #[getter(value)]
    fn value(&self) -> Option<ir::Constant> {
        self.rvalue.value().map(|constant| constant.clone().into())
    }

    #[getter(reference)]
    fn reference(&self) -> Option<ir::Reference> {
        self.rvalue
            .reference()
            .map(|reference| reference.clone().into())
    }

    #[getter(bits)]
    fn bits(&self) -> usize {
        self.rvalue.bits()
    }

    #[getter(json)]
    fn json(&self) -> PyResult<String> {
        map_err(serde_json::to_string(&self.rvalue))
    }
}

#[pyproto]
impl<'p> PyObjectProtocol<'p> for RValue {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.rvalue.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.rvalue.to_string())
    }

    fn __hash__(&self) -> PyResult<isize> {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        Ok(s.finish() as isize)
    }
}

impl From<raptor::ir::RValue<raptor::ir::Constant>> for RValue {
    fn from(rvalue: raptor::ir::RValue<raptor::ir::Constant>) -> RValue {
        RValue { rvalue }
    }
}
