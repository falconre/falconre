use super::Expression;
use crate::map_err;
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

    fn name(&self) -> &str {
        self.scalar.name()
    }

    fn bits(&self) -> usize {
        self.scalar.bits()
    }

    fn json(&self) -> PyResult<String> {
        map_err(serde_json::to_string(&self.scalar))
    }

    fn e(&self) -> Expression {
        Expression {
            expression: self.scalar.clone().into(),
        }
    }

    fn __str__(&self) -> String {
        self.scalar.to_string()
    }

    fn __repr__(&self) -> String {
        self.scalar.to_string()
    }
}

impl From<falcon::il::Scalar> for Scalar {
    fn from(scalar: falcon::il::Scalar) -> Scalar {
        Scalar { scalar }
    }
}
