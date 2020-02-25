use crate::map_err;
use pyo3::class::PyObjectProtocol;
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct Reference {
    pub(crate) reference: raptor::ir::Reference<raptor::ir::Constant>,
}

#[pymethods]
impl Reference {
    fn bits(&self) -> usize {
        self.reference.bits()
    }

    fn json(&self) -> PyResult<String> {
        map_err(serde_json::to_string(&self.reference))
    }
}

#[pyproto]
impl<'p> PyObjectProtocol<'p> for Reference {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.reference.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.reference.to_string())
    }
}

impl From<raptor::ir::Reference<raptor::ir::Constant>> for Reference {
    fn from(reference: raptor::ir::Reference<raptor::ir::Constant>) -> Reference {
        Reference { reference }
    }
}
