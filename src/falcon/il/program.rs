use crate::map_err;
use pyo3::prelude::*;

use super::Function;

#[pyclass]
pub struct Program {
    pub(crate) program: falcon::il::Program,
}

#[pymethods]
impl Program {
    fn function_by_address(&self, address: u64) -> Option<Function> {
        self.program
            .function_by_address(address)
            .map(|f| f.clone().into())
    }

    #[getter(functions)]
    fn functions(&self) -> Vec<Function> {
        self.program
            .functions()
            .into_iter()
            .map(|f| f.clone().into())
            .collect()
    }

    fn function(&self, index: usize) -> Option<Function> {
        self.program.function(index).map(|f| f.clone().into())
    }

    fn function_by_name(&self, name: &str) -> Option<Function> {
        self.program
            .function_by_name(name)
            .map(|f| f.clone().into())
    }

    #[getter(json)]
    fn json(&self) -> PyResult<String> {
        map_err(serde_json::to_string(&self.program))
    }
}

impl From<falcon::il::Program> for Program {
    fn from(program: falcon::il::Program) -> Program {
        Program { program }
    }
}
