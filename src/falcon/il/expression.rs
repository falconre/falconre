use crate::map_err;
use pyo3::class::PyObjectProtocol;
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct Expression {
    pub(crate) expression: falcon::il::Expression,
}

#[pymethods]
impl Expression {
    fn bits(&self) -> usize {
        self.expression.bits()
    }

    fn json(&self) -> PyResult<String> {
        map_err(serde_json::to_string(&self.expression))
    }
}

#[pyproto]
impl<'p> PyObjectProtocol<'p> for Expression {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.expression.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.expression.to_string())
    }
}

impl From<falcon::il::Expression> for Expression {
    fn from(expression: falcon::il::Expression) -> Expression {
        Expression { expression }
    }
}
