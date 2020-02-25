use crate::map_err;
use crate::raptor::ir;
use pyo3::class::PyObjectProtocol;
use pyo3::prelude::*;

#[pyclass]
pub struct Expression {
    pub(crate) expression: raptor::ir::Expression<raptor::ir::Constant>,
}

#[pymethods]
impl Expression {
    fn variables(&self) -> Vec<ir::Variable> {
        self.expression
            .variables()
            .into_iter()
            .map(|variable| variable.clone().into())
            .collect()
    }

    fn contains_reference(&self) -> bool {
        self.expression.contains_reference()
    }

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

impl From<raptor::ir::Expression<raptor::ir::Constant>> for Expression {
    fn from(expression: raptor::ir::Expression<raptor::ir::Constant>) -> Expression {
        Expression { expression }
    }
}
