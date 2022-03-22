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
    #[getter(address)]
    fn address(&self) -> u64 {
        self.function_entry.address()
    }

    #[getter(name)]
    fn name(&self) -> Option<String> {
        self.function_entry.name().map(|s| s.to_string())
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.function_entry.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.function_entry.to_string())
    }
}

impl From<falcon::loader::FunctionEntry> for FunctionEntry {
    fn from(function_entry: falcon::loader::FunctionEntry) -> FunctionEntry {
        FunctionEntry { function_entry }
    }
}

#[pyclass]
pub struct Symbol {
    symbol: falcon::loader::Symbol,
}

#[pymethods]
impl Symbol {
    #[getter(name)]
    fn name(&self) -> &str {
        self.symbol.name()
    }

    #[getter(address)]
    fn address(&self) -> u64 {
        self.symbol.address()
    }
}

impl From<falcon::loader::Symbol> for Symbol {
    fn from(symbol: falcon::loader::Symbol) -> Symbol {
        Symbol { symbol }
    }
}

#[pyclass]
pub struct ManualEdge {
    pub(crate) manual_edge: falcon::translator::ManualEdge,
}

#[pymethods]
impl ManualEdge {
    #[new]
    fn new(head_address: u64, tail_address: u64, condition: Option<il::Expression>) -> ManualEdge {
        ManualEdge {
            manual_edge: falcon::translator::ManualEdge::new(
                head_address,
                tail_address,
                condition.map(|e| e.expression),
            ),
        }
    }

    #[getter(head_address)]
    fn head_address(&self) -> u64 {
        self.manual_edge.head_address()
    }

    #[getter(tail_address)]
    fn tail_address(&self) -> u64 {
        self.manual_edge.tail_address()
    }

    #[getter(condition)]
    fn condition(&self) -> Option<il::Expression> {
        self.manual_edge
            .condition()
            .map(|e| il::Expression::from(e.clone()))
    }
}

impl From<falcon::translator::ManualEdge> for ManualEdge {
    fn from(manual_edge: falcon::translator::ManualEdge) -> ManualEdge {
        ManualEdge { manual_edge }
    }
}

#[pyclass]
pub struct Options {
    pub(crate) options: falcon::translator::Options,
}

#[pymethods]
impl Options {
    #[new]
    fn new() -> Options {
        Options {
            options: falcon::translator::Options::new(),
        }
    }

    fn set_unsupported_are_intrinsics(&mut self, unsupported_are_intrinsics: bool) {
        self.options
            .set_unsupported_are_intrinsics(unsupported_are_intrinsics);
    }

    #[getter(unsupported_are_intrinsics)]
    fn unsupported_are_intrinsics(&self) -> bool {
        self.options.unsupported_are_intrinsics()
    }

    fn add_manual_edge(&mut self, manual_edge: &ManualEdge) {
        self.options
            .add_manual_edge(manual_edge.manual_edge.clone());
    }

    #[getter(manual_edges)]
    fn manual_edges(&self) -> Vec<ManualEdge> {
        self.options
            .manual_edges()
            .iter()
            .map(|m| m.clone().into())
            .collect()
    }
}

trait FalconreLoader: falcon::loader::Loader {
    fn falconre_memory(&self) -> PyResult<crate::falcon::memory::backing::Memory> {
        Ok(map_err(self.memory())?.into())
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

    fn falconre_program_verbose(
        &self,
        options: &Options,
    ) -> PyResult<(il::Program, Vec<(FunctionEntry, String)>)> {
        map_err(
            self.program_verbose(&options.options)
                .map(|(program, errors)| {
                    (
                        program.into(),
                        errors
                            .into_iter()
                            .map(|(function_entry, error)| {
                                (function_entry.into(), error.to_string())
                            })
                            .collect(),
                    )
                }),
        )
    }

    fn falconre_program_recursive(&self) -> PyResult<il::Program> {
        map_err(self.program_recursive().map(|program| program.into()))
    }

    fn falconre_program_recursive_verbose(
        &self,
        options: &Options,
    ) -> PyResult<(il::Program, Vec<(FunctionEntry, String)>)> {
        map_err(
            self.program_recursive_verbose(&options.options)
                .map(|(program, errors)| {
                    (
                        program.into(),
                        errors
                            .into_iter()
                            .map(|(function_entry, error)| {
                                (function_entry.into(), error.to_string())
                            })
                            .collect(),
                    )
                }),
        )
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

    #[getter(dt_needed)]
    fn dt_needed(&self) -> Option<Vec<String>> {
        self.elf.dt_needed().ok()
    }

    #[getter(architecture)]
    fn architecture(&self) -> Architecture {
        self.elf.falconre_architecture()
    }

    #[getter(memory)]
    fn memory(&self) -> PyResult<crate::falcon::memory::backing::Memory> {
        self.elf.falconre_memory()
    }

    #[getter(program_entry)]
    fn program_entry(&self) -> u64 {
        self.elf.falconre_program_entry()
    }

    #[getter(symbols)]
    fn symbols(&self) -> Vec<Symbol> {
        self.elf.falconre_symbols()
    }

    fn function(&self, address: u64) -> PyResult<il::Function> {
        self.elf.falconre_function(address)
    }

    fn program(&self) -> PyResult<il::Program> {
        self.elf.falconre_program()
    }

    fn program_verbose(
        &self,
        options: &Options,
    ) -> PyResult<(il::Program, Vec<(FunctionEntry, String)>)> {
        self.elf.falconre_program_verbose(options)
    }

    fn program_recursive(&self) -> PyResult<il::Program> {
        self.elf.falconre_program_recursive()
    }

    fn program_recursive_verbose(
        &self,
        options: &Options,
    ) -> PyResult<(il::Program, Vec<(FunctionEntry, String)>)> {
        self.elf.falconre_program_recursive_verbose(options)
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

    #[getter(architecture)]
    fn architecture(&self) -> Architecture {
        self.pe.falconre_architecture()
    }

    #[getter(memory)]
    fn memory(&self) -> PyResult<crate::falcon::memory::backing::Memory> {
        self.pe.falconre_memory()
    }

    #[getter(program_entry)]
    fn program_entry(&self) -> u64 {
        self.pe.falconre_program_entry()
    }

    #[getter(symbols)]
    fn symbols(&self) -> Vec<Symbol> {
        self.pe.falconre_symbols()
    }

    fn function(&self, address: u64) -> PyResult<il::Function> {
        self.pe.falconre_function(address)
    }

    fn program(&self) -> PyResult<il::Program> {
        self.pe.falconre_program()
    }

    fn program_verbose(
        &self,
        options: &Options,
    ) -> PyResult<(il::Program, Vec<(FunctionEntry, String)>)> {
        self.pe.falconre_program_verbose(options)
    }

    fn program_recursive(&self) -> PyResult<il::Program> {
        self.pe.falconre_program_recursive()
    }

    fn program_recursive_verbose(
        &self,
        options: &Options,
    ) -> PyResult<(il::Program, Vec<(FunctionEntry, String)>)> {
        self.pe.falconre_program_recursive_verbose(options)
    }
}

impl From<falcon::loader::Pe> for Pe {
    fn from(pe: falcon::loader::Pe) -> Pe {
        Pe { pe }
    }
}
