use crate::map_err;
use pyo3::prelude::*;
use pyo3::{wrap_pyfunction, wrap_pymodule};

pub mod analysis;
pub mod ir;

#[pyfunction]
fn translate_function(
    elf: &crate::falcon::loader::Elf,
    function: &crate::falcon::il::Function,
) -> PyResult<crate::raptor::ir::Function> {
    use falcon::loader::Loader;

    let architecture = elf.elf.architecture();

    let program_translator = map_err(raptor::translator::ProgramTranslator::new(
        architecture.box_clone(),
        architecture.calling_convention(),
        &elf.elf,
    ))?;

    map_err(
        program_translator
            .function_translator()
            .translate_function(&function.function),
    )
    .map(|function| function.into())
}

#[pymodule(ir)]
fn raptor_ir_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ir::Block>()?;
    m.add_class::<ir::Constant>()?;
    m.add_class::<ir::ControlFlowGraph>()?;
    m.add_class::<ir::Dereference>()?;
    m.add_class::<ir::Edge>()?;
    m.add_class::<ir::Expression>()?;
    m.add_class::<ir::Function>()?;
    m.add_class::<ir::Instruction>()?;
    m.add_class::<ir::LValue>()?;
    m.add_class::<ir::Operation>()?;
    m.add_class::<ir::Program>()?;
    m.add_class::<ir::Reference>()?;
    m.add_class::<ir::RValue>()?;
    m.add_class::<ir::Scalar>()?;
    m.add_class::<ir::StackVariable>()?;
    m.add_class::<ir::Variable>()?;
    Ok(())
}

#[pymodule(raptor)]
pub fn raptor_module(py: Python, m: &PyModule) -> PyResult<()> {
    use analysis::PyInit_raptor_analysis;

    m.add_wrapped(wrap_pymodule!(ir))?;
    m.add("analysis", wrap_pymodule!(raptor_analysis)(py))?;
    m.add_wrapped(wrap_pyfunction!(translate_function))?;
    Ok(())
}
