use crate::map_err;
use pyo3::class::PyObjectProtocol;
use pyo3::prelude::*;

#[pyclass]
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

    fn name(&self) -> &str {
        self.scalar.name()
    }

    fn bits(&self) -> usize {
        self.scalar.bits()
    }

    fn ssa(&self) -> Option<usize> {
        self.scalar.ssa()
    }

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
}

impl From<raptor::ir::Scalar> for Scalar {
    fn from(scalar: raptor::ir::Scalar) -> Scalar {
        Scalar { scalar }
    }
}
