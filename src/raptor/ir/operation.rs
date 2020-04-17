use crate::raptor::ir;
use pyo3::class::PyObjectProtocol;
use pyo3::prelude::*;

#[pyclass]
pub struct Operation {
    pub(crate) operation: raptor::ir::Operation<raptor::ir::Constant>,
}

#[pymethods]
impl Operation {
    fn r#type(&self) -> &str {
        match self.operation {
            raptor::ir::Operation::Assign { .. } => "assign",
            raptor::ir::Operation::Store { .. } => "store",
            raptor::ir::Operation::Load { .. } => "load",
            raptor::ir::Operation::Branch { .. } => "branch",
            raptor::ir::Operation::Call(_) => "call",
            raptor::ir::Operation::Intrinsic(_) => "intrinsic",
            raptor::ir::Operation::Return(_) => "return",
            raptor::ir::Operation::Nop => "nop",
        }
    }

    fn variables_written(&self) -> Option<Vec<ir::Variable>> {
        self.operation
            .variables_written()
            .map(|vw| vw.into_iter().map(|v| v.clone().into()).collect())
    }

    fn variables_read(&self) -> Option<Vec<ir::Variable>> {
        self.operation
            .variables_read()
            .map(|vw| vw.into_iter().map(|v| v.clone().into()).collect())
    }

    fn call(&self) -> Option<ir::Call> {
        self.operation.call().map(|call| call.clone().into())
    }

    fn src(&self) -> Option<ir::Expression> {
        self.operation.src().map(|e| e.clone().into())
    }

    fn dst(&self) -> Option<ir::Variable> {
        self.operation.dst().map(|e| e.clone().into())
    }

    fn index(&self) -> Option<ir::Expression> {
        self.operation.index().map(|e| e.clone().into())
    }
}

#[pyproto]
impl<'p> PyObjectProtocol<'p> for Operation {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.operation.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.operation.to_string())
    }
}

impl From<raptor::ir::Operation<raptor::ir::Constant>> for Operation {
    fn from(operation: raptor::ir::Operation<raptor::ir::Constant>) -> Operation {
        Operation { operation }
    }
}
