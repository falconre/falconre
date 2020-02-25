use crate::map_err;
use pyo3::class::PyObjectProtocol;
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct Dereference {
    pub(crate) dereference: raptor::ir::Dereference<raptor::ir::Constant>,
}

#[pymethods]
impl Dereference {
    fn bits(&self) -> usize {
        self.dereference.bits()
    }

    fn json(&self) -> PyResult<String> {
        map_err(serde_json::to_string(&self.dereference))
    }
}

#[pyproto]
impl<'p> PyObjectProtocol<'p> for Dereference {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.dereference.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.dereference.to_string())
    }
}

impl From<raptor::ir::Dereference<raptor::ir::Constant>> for Dereference {
    fn from(dereference: raptor::ir::Dereference<raptor::ir::Constant>) -> Dereference {
        Dereference { dereference }
    }
}
