use super::Expression;
use crate::map_err;
use pyo3::class::PyObjectProtocol;
use pyo3::prelude::*;

#[pyclass]
pub struct Scalar {
    pub(crate) scalar: falcon::il::Scalar,
}

#[pymethods]
impl Scalar {
    #[new]
    fn new(name: String, bits: usize) -> Scalar {
        Scalar {
            scalar: falcon::il::Scalar::new(name, bits),
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

    #[getter(json)]
    fn json(&self) -> PyResult<String> {
        map_err(serde_json::to_string(&self.scalar))
    }

    #[getter(e)]
    fn e(&self) -> Expression {
        Expression {
            expression: self.scalar.clone().into(),
        }
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

impl From<falcon::il::Scalar> for Scalar {
    fn from(scalar: falcon::il::Scalar) -> Scalar {
        Scalar { scalar }
    }
}
