use crate::map_err;
use crate::raptor::ir;
use pyo3::class::PyObjectProtocol;
use pyo3::prelude::*;

#[pyclass]
pub struct Variable {
    pub(crate) variable: raptor::ir::Variable,
}

#[pymethods]
impl Variable {
    fn scalar(&self) -> Option<ir::Scalar> {
        self.variable.scalar().map(|scalar| scalar.clone().into())
    }

    fn stack_variable(&self) -> Option<ir::StackVariable> {
        self.variable
            .stack_variable()
            .map(|stack_variable| stack_variable.clone().into())
    }

    fn bits(&self) -> usize {
        self.variable.bits()
    }

    fn json(&self) -> PyResult<String> {
        map_err(serde_json::to_string(&self.variable))
    }
}

#[pyproto]
impl<'p> PyObjectProtocol<'p> for Variable {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.variable.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.variable.to_string())
    }
}

impl From<raptor::ir::Variable> for Variable {
    fn from(variable: raptor::ir::Variable) -> Variable {
        Variable { variable }
    }
}
