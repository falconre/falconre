use pyo3::class::basic::CompareOp;
use pyo3::class::PyObjectProtocol;
use pyo3::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use super::{Function, Program};

#[pyclass]
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct ProgramLocation {
    pub(crate) program_location: raptor::ir::ProgramLocation,
}

#[pymethods]
impl ProgramLocation {
    #[new]
    fn new(function_index: usize, function_location: &FunctionLocation) -> ProgramLocation {
        ProgramLocation {
            program_location: raptor::ir::ProgramLocation::new(
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
        raptor::ir::RefProgramLocation::from_address(&program.program, address).map(|rpl| {
            ProgramLocation {
                program_location: rpl.into(),
            }
        })
    }

    #[staticmethod]
    fn from_function(function: &Function) -> PyResult<Option<ProgramLocation>> {
        Ok(
            raptor::ir::RefProgramLocation::from_function(&function.function).map(|result| ProgramLocation {
                    program_location: result.into(),
                }),
        )
    }
}

#[pyproto]
impl<'p> PyObjectProtocol<'p> for ProgramLocation {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.program_location.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.program_location.to_string())
    }

    fn __hash__(&self) -> PyResult<isize> {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        Ok(s.finish() as isize)
    }

    fn __richcmp__(&self, other: ProgramLocation, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Eq => Ok(*self == other),
            _ => Ok(false),
        }
    }
}

impl From<raptor::ir::ProgramLocation> for ProgramLocation {
    fn from(program_location: raptor::ir::ProgramLocation) -> ProgramLocation {
        ProgramLocation { program_location }
    }
}

#[pyclass]
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct FunctionLocation {
    pub(crate) function_location: raptor::ir::FunctionLocation,
}

#[pymethods]
impl FunctionLocation {
    #[staticmethod]
    fn from_instruction(block_index: usize, instruction_index: usize) -> FunctionLocation {
        FunctionLocation {
            function_location: raptor::ir::FunctionLocation::Instruction(
                block_index,
                instruction_index,
            ),
        }
    }

    fn block_index(&self) -> Option<usize> {
        self.function_location.block_index()
    }

    fn instruction_index(&self) -> Option<usize> {
        self.function_location.instruction_index()
    }
}

#[pyproto]
impl<'p> PyObjectProtocol<'p> for FunctionLocation {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.function_location.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.function_location.to_string())
    }

    fn __hash__(&self) -> PyResult<isize> {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        Ok(s.finish() as isize)
    }

    fn __richcmp__(&self, other: FunctionLocation, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Eq => Ok(*self == other),
            _ => Ok(false),
        }
    }
}

impl From<raptor::ir::FunctionLocation> for FunctionLocation {
    fn from(function_location: raptor::ir::FunctionLocation) -> FunctionLocation {
        FunctionLocation { function_location }
    }
}
