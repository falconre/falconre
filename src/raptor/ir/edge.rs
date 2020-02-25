use crate::raptor::ir;
use pyo3::class::PyObjectProtocol;
use pyo3::prelude::*;

#[pyclass]
pub struct Edge {
    pub(crate) edge: raptor::ir::Edge<raptor::ir::Constant>,
}

#[pymethods]
impl Edge {
    fn head(&self) -> usize {
        self.edge.head()
    }

    fn tail(&self) -> usize {
        self.edge.tail()
    }

    fn condition(&self) -> Option<ir::Expression> {
        self.edge.condition().map(|e| e.clone().into())
    }
}

#[pyproto]
impl<'p> PyObjectProtocol<'p> for Edge {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.edge.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.edge.to_string())
    }
}

impl From<raptor::ir::Edge<raptor::ir::Constant>> for Edge {
    fn from(edge: raptor::ir::Edge<raptor::ir::Constant>) -> Edge {
        Edge { edge }
    }
}
