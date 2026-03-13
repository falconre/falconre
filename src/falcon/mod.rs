pub mod analysis;
pub mod architecture;
pub mod il;
pub mod loader;
pub mod memory;

use pyo3::prelude::*;

pub fn register_falcon(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new(parent.py(), "falcon")?;

    analysis::register_analysis(&m)?;
    register_architecture(&m)?;
    register_il(&m)?;
    register_loader(&m)?;
    register_memory(&m)?;

    parent.add_submodule(&m)?;
    Ok(())
}

fn register_architecture(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new(parent.py(), "architecture")?;
    m.add_class::<architecture::Endian>()?;
    m.add_class::<architecture::Architecture>()?;
    parent.add_submodule(&m)?;
    Ok(())
}

fn register_il(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new(parent.py(), "il")?;
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
    parent.add_submodule(&m)?;
    Ok(())
}

fn register_loader(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new(parent.py(), "loader")?;
    m.add_class::<loader::Elf>()?;
    m.add_class::<loader::FunctionEntry>()?;
    m.add_class::<loader::Symbol>()?;
    m.add_class::<loader::Pe>()?;
    parent.add_submodule(&m)?;
    Ok(())
}

fn register_memory(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new(parent.py(), "memory")?;
    m.add_class::<memory::backing::Section>()?;
    m.add_class::<memory::backing::Memory>()?;
    m.add_class::<memory::paged::ConstantMemory>()?;
    m.add_class::<memory::paged::ExpressionMemory>()?;
    parent.add_submodule(&m)?;
    Ok(())
}
