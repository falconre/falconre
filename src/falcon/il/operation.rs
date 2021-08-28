use crate::map_err;
use pyo3::class::PyObjectProtocol;
use pyo3::prelude::*;

use super::Scalar;

#[pyclass]
pub struct Operation {
    pub(crate) operation: falcon::il::Operation,
}

#[pymethods]
impl Operation {
    #[getter(is_assign)]
    fn is_assign(&self) -> bool {
        self.operation.is_assign()
    }

    #[getter(is_store)]
    fn is_store(&self) -> bool {
        self.operation.is_store()
    }

    #[getter(is_load)]
    fn is_load(&self) -> bool {
        self.operation.is_load()
    }

    #[getter(is_branch)]
    fn is_branch(&self) -> bool {
        self.operation.is_branch()
    }

    #[getter(is_intrinsic)]
    fn is_intrinsic(&self) -> bool {
        self.operation.is_intrinsic()
    }

    #[getter(is_nop)]
    fn is_nop(&self) -> bool {
        self.operation.is_nop()
    }

    #[getter(scalars_read)]
    fn scalars_read(&self) -> Option<Vec<Scalar>> {
        self.operation.scalars_read().map(|scalars_read| {
            scalars_read
                .into_iter()
                .map(|s| s.clone().into())
                .collect::<Vec<Scalar>>()
        })
    }

    #[getter(scalars_written)]
    fn scalars_written(&self) -> Option<Vec<Scalar>> {
        self.operation.scalars_written().map(|scalars_read| {
            scalars_read
                .into_iter()
                .map(|s| s.clone().into())
                .collect::<Vec<Scalar>>()
        })
    }

    #[getter(json)]
    fn json(&self) -> PyResult<String> {
        map_err(serde_json::to_string(&self.operation))
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

impl From<falcon::il::Operation> for Operation {
    fn from(operation: falcon::il::Operation) -> Operation {
        Operation { operation }
    }
}
