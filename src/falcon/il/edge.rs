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

    fn __str__(&self) -> String {
        self.edge.to_string()
    }

    fn __repr__(&self) -> String {
        self.edge.to_string()
    }
}

impl From<falcon::il::Edge> for Edge {
    fn from(edge: falcon::il::Edge) -> Edge {
        Edge { edge }
    }
}
