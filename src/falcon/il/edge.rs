use pyo3::class::PyObjectProtocol;
use pyo3::prelude::*;

use super::Expression;

#[pyclass]
pub struct Edge {
    edge: falcon::il::Edge,
}

#[pymethods]
impl Edge {
    fn condition(&self) -> Option<Expression> {
        self.edge.condition().map(|c| c.clone().into())
    }

    fn head(&self) -> usize {
        self.edge.head()
    }

    fn tail(&self) -> usize {
        self.edge.tail()
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

impl From<falcon::il::Edge> for Edge {
    fn from(edge: falcon::il::Edge) -> Edge {
        Edge { edge }
    }
}
