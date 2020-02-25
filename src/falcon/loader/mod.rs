use pyo3::class::PyObjectProtocol;
use pyo3::prelude::*;

use crate::falcon::architecture::Architecture;
use crate::falcon::il;
use crate::map_err;

#[pyclass]
pub struct FunctionEntry {
    function_entry: falcon::loader::FunctionEntry,
}

#[pymethods]
impl FunctionEntry {
    fn address(&self) -> u64 {
        self.function_entry.address()
    }

    fn name(&self) -> Option<String> {
        self.function_entry.name().map(|s| s.to_string())
    }
}

impl From<falcon::loader::FunctionEntry> for FunctionEntry {
    fn from(function_entry: falcon::loader::FunctionEntry) -> FunctionEntry {
        FunctionEntry { function_entry }
    }
}

#[pyproto]
impl<'p> PyObjectProtocol<'p> for FunctionEntry {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.function_entry.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.function_entry.to_string())
    }
}

#[pyclass]
pub struct Symbol {
    symbol: falcon::loader::Symbol,
}

#[pymethods]
impl Symbol {
    fn name(&self) -> &str {
        self.symbol.name()
    }

    fn address(&self) -> u64 {
        self.symbol.address()
    }
}

impl From<falcon::loader::Symbol> for Symbol {
    fn from(symbol: falcon::loader::Symbol) -> Symbol {
        Symbol { symbol }
    }
}

trait FalconreLoader: falcon::loader::Loader {
    fn falconre_memory(&self) -> PyResult<crate::falcon::memory::backing::Memory> {
        Ok(self
            .memory()
            .map_err(|e| pyo3::exceptions::Exception::py_err(format!("{}", e)))?
            .into())
    }

    fn falconre_architecture(&self) -> Architecture {
        Architecture {
            architecture: self.architecture().box_clone(),
        }
    }

    fn falconre_program_entry(&self) -> u64 {
        self.program_entry()
    }

    fn falconre_symbols(&self) -> Vec<Symbol> {
        self.symbols().into_iter().map(|s| s.into()).collect()
    }

    fn falconre_function(&self, address: u64) -> PyResult<il::Function> {
        map_err(self.function(address)).map(|function| function.into())
    }

    fn falconre_program(&self) -> PyResult<il::Program> {
        map_err(self.program().map(|program| program.into()))
    }

    fn falconre_program_verbose(&self) -> PyResult<(il::Program, Vec<(FunctionEntry, String)>)> {
        map_err(self.program_verbose().map(|(program, errors)| {
            (
                program.into(),
                errors
                    .into_iter()
                    .map(|(function_entry, error)| (function_entry.into(), error.to_string()))
                    .collect(),
            )
        }))
    }

    fn falconre_program_recursive(&self) -> PyResult<il::Program> {
        map_err(self.program_recursive().map(|program| program.into()))
    }

    fn falconre_program_recursive_verbose(
        &self,
    ) -> PyResult<(il::Program, Vec<(FunctionEntry, String)>)> {
        map_err(self.program_recursive_verbose().map(|(program, errors)| {
            (
                program.into(),
                errors
                    .into_iter()
                    .map(|(function_entry, error)| (function_entry.into(), error.to_string()))
                    .collect(),
            )
        }))
    }
}

impl FalconreLoader for falcon::loader::Elf {}

#[pyclass]
pub struct Elf {
    pub(crate) elf: falcon::loader::Elf,
}

#[pymethods]
impl Elf {
    #[new]
    fn new(filename: &str) -> PyResult<Elf> {
        let elf = map_err(falcon::loader::Elf::from_file(std::path::Path::new(
            filename,
        )))?;
        Ok(Elf { elf })
    }

    fn add_user_function(&mut self, address: u64) {
        self.elf.add_user_function(address);
    }

    fn dt_needed(&self) -> Option<Vec<String>> {
        self.elf.dt_needed().ok()
    }

    fn architecture(&self) -> Architecture {
        self.elf.falconre_architecture()
    }

    fn memory(&self) -> PyResult<crate::falcon::memory::backing::Memory> {
        self.elf.falconre_memory()
    }

    fn program_entry(&self) -> u64 {
        self.elf.falconre_program_entry()
    }

    fn symbols(&self) -> Vec<Symbol> {
        self.elf.falconre_symbols()
    }

    fn function(&self, address: u64) -> PyResult<il::Function> {
        self.elf.falconre_function(address)
    }

    fn program(&self) -> PyResult<il::Program> {
        self.elf.falconre_program()
    }

    fn program_verbose(&self) -> PyResult<(il::Program, Vec<(FunctionEntry, String)>)> {
        self.elf.falconre_program_verbose()
    }

    fn program_recursive(&self) -> PyResult<il::Program> {
        self.elf.falconre_program_recursive()
    }

    fn program_recursive_verbose(&self) -> PyResult<(il::Program, Vec<(FunctionEntry, String)>)> {
        self.elf.falconre_program_recursive_verbose()
    }
}

impl From<falcon::loader::Elf> for Elf {
    fn from(elf: falcon::loader::Elf) -> Elf {
        Elf { elf }
    }
}

impl FalconreLoader for falcon::loader::Pe {}

#[pyclass]
pub struct Pe {
    pub(crate) pe: falcon::loader::Pe,
}

#[pymethods]
impl Pe {
    #[new]
    fn new(filename: &str) -> PyResult<Pe> {
        let pe = map_err(falcon::loader::Pe::from_file(std::path::Path::new(
            filename,
        )))?;
        Ok(Pe { pe })
    }

    fn architecture(&self) -> Architecture {
        self.pe.falconre_architecture()
    }

    fn memory(&self) -> PyResult<crate::falcon::memory::backing::Memory> {
        self.pe.falconre_memory()
    }

    fn program_entry(&self) -> u64 {
        self.pe.falconre_program_entry()
    }

    fn symbols(&self) -> Vec<Symbol> {
        self.pe.falconre_symbols()
    }

    fn function(&self, address: u64) -> PyResult<il::Function> {
        self.pe.falconre_function(address)
    }

    fn program(&self) -> PyResult<il::Program> {
        self.pe.falconre_program()
    }

    fn program_verbose(&self) -> PyResult<(il::Program, Vec<(FunctionEntry, String)>)> {
        self.pe.falconre_program_verbose()
    }

    fn program_recursive(&self) -> PyResult<il::Program> {
        self.pe.falconre_program_recursive()
    }

    fn program_recursive_verbose(&self) -> PyResult<(il::Program, Vec<(FunctionEntry, String)>)> {
        self.pe.falconre_program_recursive_verbose()
    }
}

impl From<falcon::loader::Pe> for Pe {
    fn from(pe: falcon::loader::Pe) -> Pe {
        Pe { pe }
    }
}
