use crate::map_err;
use pyo3::prelude::*;

use super::{Function, Program};

#[pyclass(from_py_object)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ProgramLocation {
    pub(crate) program_location: falcon::il::ProgramLocation,
}

#[pymethods]
impl ProgramLocation {
    #[new]
    fn new(function_index: Option<usize>, function_location: &FunctionLocation) -> ProgramLocation {
        ProgramLocation {
            program_location: falcon::il::ProgramLocation::new(
                function_index,
                function_location.function_location.clone(),
            ),
        }
    }

    fn function_location(&self) -> FunctionLocation {
        FunctionLocation {
            function_location: self.program_location.function_location().clone(),
        }
    }

    #[staticmethod]
    fn from_address(program: &Program, address: u64) -> Option<ProgramLocation> {
        falcon::il::RefProgramLocation::from_address(&program.program, address).map(|rpl| {
            ProgramLocation {
                program_location: rpl.into(),
            }
        })
    }

    #[staticmethod]
    fn from_function(function: &Function) -> PyResult<Option<ProgramLocation>> {
        Ok(
            match falcon::il::RefProgramLocation::from_function(&function.function) {
                Some(result) => Some(ProgramLocation {
                    program_location: map_err(result)?.into(),
                }),
                None => None,
            },
        )
    }

    fn __str__(&self) -> String {
        self.program_location.to_string()
    }

    fn __repr__(&self) -> String {
        self.program_location.to_string()
    }
}

impl From<falcon::il::ProgramLocation> for ProgramLocation {
    fn from(program_location: falcon::il::ProgramLocation) -> ProgramLocation {
        ProgramLocation { program_location }
    }
}

#[pyclass]
pub struct FunctionLocation {
    pub(crate) function_location: falcon::il::FunctionLocation,
}

#[pymethods]
impl FunctionLocation {
    fn block_index(&self) -> Option<usize> {
        self.function_location.block_index()
    }

    fn instruction_index(&self) -> Option<usize> {
        self.function_location.instruction_index()
    }

    fn __str__(&self) -> String {
        self.function_location.to_string()
    }

    fn __repr__(&self) -> String {
        self.function_location.to_string()
    }
}

impl From<falcon::il::FunctionLocation> for FunctionLocation {
    fn from(function_location: falcon::il::FunctionLocation) -> FunctionLocation {
        FunctionLocation { function_location }
    }
}
