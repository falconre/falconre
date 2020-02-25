use crate::raptor::ir;
use pyo3::class::PyObjectProtocol;
use pyo3::prelude::*;

#[pyclass]
pub struct CallTarget {
    pub(crate) call_target: raptor::ir::CallTarget<raptor::ir::Constant>,
}

#[pymethods]
impl CallTarget {
    fn r#type(&self) -> &str {
        match self.call_target {
            raptor::ir::CallTarget::Expression(_) => "expression",
            raptor::ir::CallTarget::Symbol(_) => "symbol",
            raptor::ir::CallTarget::FunctionId(_) => "function_id",
        }
    }

    fn expression(&self) -> Option<ir::Expression> {
        self.call_target
            .expression()
            .map(|expression| expression.clone().into())
    }

    fn symbol(&self) -> Option<&str> {
        self.call_target.symbol()
    }

    fn function_id(&self) -> Option<usize> {
        self.call_target.function_id()
    }
}

impl From<raptor::ir::CallTarget<raptor::ir::Constant>> for CallTarget {
    fn from(call_target: raptor::ir::CallTarget<raptor::ir::Constant>) -> CallTarget {
        CallTarget { call_target }
    }
}

#[pyclass]
pub struct Call {
    pub(crate) call: raptor::ir::Call<raptor::ir::Constant>,
}

#[pymethods]
impl Call {
    fn target(&self) -> CallTarget {
        self.call.target().clone().into()
    }

    fn arguments(&self) -> Option<Vec<ir::Expression>> {
        self.call.arguments().map(|arguments| {
            arguments
                .into_iter()
                .map(|argument| argument.clone().into())
                .collect()
        })
    }
}

#[pyproto]
impl<'p> PyObjectProtocol<'p> for Call {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.call.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.call.to_string())
    }
}

impl From<raptor::ir::Call<raptor::ir::Constant>> for Call {
    fn from(call: raptor::ir::Call<raptor::ir::Constant>) -> Call {
        Call { call }
    }
}
