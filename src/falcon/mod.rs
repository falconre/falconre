pub mod analysis;
pub mod architecture;
pub mod il;
pub mod loader;
pub mod memory;

use pyo3::prelude::*;

fn register_architecture_module(py: Python, parent_module: &PyModule) -> PyResult<()> {
    let m = PyModule::new(py, "architecture")?;
    m.add_class::<architecture::Endian>()?;
    m.add_class::<architecture::Architecture>()?;
    parent_module.add_submodule(m)?;
    Ok(())
}

fn register_il_module(py: Python, parent_module: &PyModule) -> PyResult<()> {
    let m = PyModule::new(py, "il")?;
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
    parent_module.add_submodule(m)?;
    Ok(())
}

fn register_loader_module(py: Python, parent_module: &PyModule) -> PyResult<()> {
    let m = PyModule::new(py, "loader")?;
    m.add_class::<loader::Elf>()?;
    m.add_class::<loader::FunctionEntry>()?;
    m.add_class::<loader::Symbol>()?;
    m.add_class::<loader::Pe>()?;
    parent_module.add_submodule(m)?;
    Ok(())
}

fn register_memory_module(py: Python, parent_module: &PyModule) -> PyResult<()> {
    let m = PyModule::new(py, "memory")?;
    m.add_class::<memory::backing::Section>()?;
    m.add_class::<memory::backing::Memory>()?;
    m.add_class::<memory::paged::ConstantMemory>()?;
    m.add_class::<memory::MemoryPermissions>()?;
    m.add_class::<memory::paged::ExpressionMemory>()?;
    parent_module.add_submodule(m)?;
    Ok(())
}

#[pymodule]
#[pyo3(name = "falcon")]
pub fn register_falcon_module(py: Python, parent_module: &PyModule) -> PyResult<()> {
    let m = PyModule::new(py, "falcon")?;

    register_architecture_module(py, m)?;
    register_il_module(py, m)?;
    register_loader_module(py, m)?;
    register_memory_module(py, m)?;
    analysis::register_analysis_module(py, m)?;

    parent_module.add_submodule(m)?;

    Ok(())
}
