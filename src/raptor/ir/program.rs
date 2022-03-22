use crate::map_err;
use crate::raptor::ir;
use pyo3::prelude::*;

#[pyclass]
pub struct Program {
    pub(crate) program: raptor::ir::Program<raptor::ir::Constant>,
}

#[pymethods]
impl Program {
    #[staticmethod]
    fn from_il(program: &crate::falcon::il::Program) -> PyResult<Program> {
        map_err(raptor::ir::Program::<raptor::ir::Constant>::from_il(
            &program.program,
        ))
        .map(|program| program.into())
    }

    #[getter(functions)]
    fn functions(&self) -> Vec<ir::Function> {
        self.program
            .functions()
            .into_iter()
            .map(|function| function.clone().into())
            .collect()
    }
}

impl From<raptor::ir::Program<raptor::ir::Constant>> for Program {
    fn from(program: raptor::ir::Program<raptor::ir::Constant>) -> Program {
        Program { program }
    }
}
