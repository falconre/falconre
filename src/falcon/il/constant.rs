use super::Expression;
use crate::map_err;
use pyo3::prelude::*;

#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct Constant {
    pub(crate) constant: falcon::il::Constant,
}

#[pymethods]
impl Constant {
    #[new]
    fn new(value: u64, bits: usize) -> Constant {
        Constant {
            constant: falcon::il::Constant::new(value, bits),
        }
    }

    fn bits(&self) -> usize {
        self.constant.bits()
    }

    fn value_u64(&self) -> Option<u64> {
        self.constant.value_u64()
    }

    fn e(&self) -> Expression {
        Expression {
            expression: self.constant.clone().into(),
        }
    }

    fn json(&self) -> PyResult<String> {
        map_err(serde_json::to_string(&self.constant))
    }

    fn __str__(&self) -> String {
        self.constant.to_string()
    }

    fn __repr__(&self) -> String {
        self.constant.to_string()
    }
}

impl From<falcon::il::Constant> for Constant {
    fn from(constant: falcon::il::Constant) -> Constant {
        Constant { constant }
    }
}
