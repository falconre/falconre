use crate::raptor::ir;
use pyo3::class::PyObjectProtocol;
use pyo3::prelude::*;

#[pyclass]
#[derive(Eq, Hash, PartialEq)]
pub struct Instruction {
    pub(crate) instruction: raptor::ir::Instruction<raptor::ir::Constant>,
}

#[pymethods]
impl Instruction {
    #[getter(index)]
    fn index(&self) -> usize {
        self.instruction.index()
    }

    #[getter(operation)]
    fn operation(&self) -> ir::Operation {
        self.instruction.operation().clone().into()
    }

    #[getter(address)]
    fn address(&self) -> Option<u64> {
        self.instruction.address()
    }

    #[getter(variables)]
    fn variables(&self) -> Option<Vec<ir::Variable>> {
        self.instruction.variables().map(|variables| {
            variables
                .into_iter()
                .map(|variable| variable.clone().into())
                .collect()
        })
    }

    #[getter(variables_read)]
    fn variables_read(&self) -> Option<Vec<ir::Variable>> {
        self.instruction.variables_read().map(|variables| {
            variables
                .into_iter()
                .map(|variable| variable.clone().into())
                .collect()
        })
    }

    #[getter(variables_written)]
    fn variables_written(&self) -> Option<Vec<ir::Variable>> {
        self.instruction.variables_written().map(|variables| {
            variables
                .into_iter()
                .map(|variable| variable.clone().into())
                .collect()
        })
    }
}

#[pyproto]
impl<'p> PyObjectProtocol<'p> for Instruction {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.instruction.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.instruction.to_string())
    }

    fn __hash__(&self) -> PyResult<isize> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        Ok(s.finish() as isize)
    }
}

impl From<raptor::ir::Instruction<raptor::ir::Constant>> for Instruction {
    fn from(instruction: raptor::ir::Instruction<raptor::ir::Constant>) -> Instruction {
        Instruction { instruction }
    }
}
