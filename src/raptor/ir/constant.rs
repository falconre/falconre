use crate::map_err;
use pyo3::class::PyObjectProtocol;
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Constant {
    pub(crate) constant: raptor::ir::Constant,
}

#[pymethods]
impl Constant {
    #[new]
    fn new(value: u64, bits: usize) -> Constant {
        Constant {
            constant: raptor::ir::Constant::new(value, bits),
        }
    }

    #[getter(bits)]
    fn bits(&self) -> usize {
        self.constant.bits()
    }

    #[getter(value_u64)]
    fn value_u64(&self) -> Option<u64> {
        self.constant.value_u64()
    }

    #[getter(value_i64)]
    fn value_i64(&self) -> Option<i64> {
        self.constant.value_i64()
    }

    #[getter(json)]
    fn json(&self) -> PyResult<String> {
        map_err(serde_json::to_string(&self.constant))
    }
}

#[pyproto]
impl<'p> PyObjectProtocol<'p> for Constant {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.constant.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.constant.to_string())
    }

    fn __hash__(&self) -> PyResult<isize> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        Ok(s.finish() as isize)
    }
}

impl From<raptor::ir::Constant> for Constant {
    fn from(constant: raptor::ir::Constant) -> Constant {
        Constant { constant }
    }
}
