use crate::map_err;
use crate::raptor::ir;
use pyo3::class::PyObjectProtocol;
use pyo3::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[pyclass]
#[derive(Eq, Hash, PartialEq)]
pub struct Scalar {
    pub(crate) scalar: raptor::ir::Scalar,
}

#[pymethods]
impl Scalar {
    #[new]
    fn new(name: String, bits: usize) -> Scalar {
        Scalar {
            scalar: raptor::ir::Scalar::new(name, bits),
        }
    }

    #[getter(name)]
    fn name(&self) -> &str {
        self.scalar.name()
    }

    #[getter(bits)]
    fn bits(&self) -> usize {
        self.scalar.bits()
    }

    #[getter(ssa)]
    fn ssa(&self) -> Option<usize> {
        self.scalar.ssa()
    }

    #[getter(variable)]
    fn variable(&self) -> ir::Variable {
        ir::Variable {
            variable: self.scalar.clone().into(),
        }
    }

    #[getter(json)]
    fn json(&self) -> PyResult<String> {
        map_err(serde_json::to_string(&self.scalar))
    }
}

#[pyproto]
impl<'p> PyObjectProtocol<'p> for Scalar {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.scalar.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.scalar.to_string())
    }

    fn __hash__(&self) -> PyResult<isize> {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        Ok(s.finish() as isize)
    }
}

impl From<raptor::ir::Scalar> for Scalar {
    fn from(scalar: raptor::ir::Scalar) -> Scalar {
        Scalar { scalar }
    }
}
