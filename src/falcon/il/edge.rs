use pyo3::prelude::*;

use super::Expression;

#[pyclass]
pub struct Edge {
    edge: falcon::il::Edge,
}

#[pymethods]
impl Edge {
    #[getter(condition)]
    fn condition(&self) -> Option<Expression> {
        self.edge.condition().map(|c| c.clone().into())
    }

    #[getter(head)]
    fn head(&self) -> usize {
        self.edge.head()
    }

    #[getter(tail)]
    fn tail(&self) -> usize {
        self.edge.tail()
    }

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
