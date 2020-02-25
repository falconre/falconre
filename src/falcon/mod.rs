pub mod analysis;
pub mod architecture;
pub mod il;
pub mod loader;
pub mod memory;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;

#[pymodule(architecture)]
fn falcon_architecture_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<architecture::Endian>()?;
    m.add_class::<architecture::Architecture>()?;
    Ok(())
}

#[pymodule(il)]
fn falcon_il_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<il::Block>()?;
    m.add_class::<il::Constant>()?;
    m.add_class::<il::ControlFlowGraph>()?;
    m.add_class::<il::Edge>()?;
    m.add_class::<il::Expression>()?;
    m.add_class::<il::Function>()?;
    m.add_class::<il::FunctionLocation>()?;
    m.add_class::<il::Instruction>()?;
    m.add_class::<il::Operation>()?;
    m.add_class::<il::Program>()?;
    m.add_class::<il::ProgramLocation>()?;
    m.add_class::<il::Scalar>()?;
    Ok(())
}

#[pymodule(loader)]
fn falcon_loader_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<loader::Elf>()?;
    m.add_class::<loader::FunctionEntry>()?;
    m.add_class::<loader::Symbol>()?;
    m.add_class::<loader::Pe>()?;
    Ok(())
}

#[pymodule(memory)]
fn falcon_memory_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<memory::backing::Section>()?;
    m.add_class::<memory::backing::Memory>()?;
    m.add_class::<memory::paged::ConstantMemory>()?;
    m.add_class::<memory::paged::ExpressionMemory>()?;
    Ok(())
}

#[pymodule(falcon)]
pub fn falcon_module(_py: Python, m: &PyModule) -> PyResult<()> {
    use crate::falcon::analysis::PyInit_analysis;

    m.add_wrapped(wrap_pymodule!(analysis))?;
    m.add_wrapped(wrap_pymodule!(architecture))?;
    m.add_wrapped(wrap_pymodule!(il))?;
    m.add_wrapped(wrap_pymodule!(loader))?;
    m.add_wrapped(wrap_pymodule!(memory))?;
    Ok(())
}
