use crate::map_err;
use pyo3::prelude::*;
use pyo3::{wrap_pyfunction, wrap_pymodule};
use rayon::prelude::*;

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

#[pyfunction]
fn translate_program_parallel(
    elf: &crate::falcon::loader::Elf,
    program: &crate::falcon::il::Program,
) -> PyResult<crate::raptor::ir::Program> {
    use falcon::loader::Loader;

    fn translate(
        function_translator: raptor::translator::FunctionTranslator,
        program: &crate::falcon::il::Program,
    ) -> PyResult<Vec<raptor::ir::Function<raptor::ir::Constant>>> {
        let functions: Result<
            Vec<raptor::ir::Function<raptor::ir::Constant>>,
            raptor::error::Error,
        > = program
            .program
            .functions()
            .par_iter()
            .map(|function| function_translator.translate_function(function))
            .try_fold(
                Vec::new,
                |mut functions, function| {
                    functions.push(function?);
                    Ok(functions)
                },
            )
            .try_reduce(
                Vec::new,
                |mut l, mut r| {
                    l.append(&mut r);
                    Ok(l)
                },
            );
        map_err(functions)
    }

    let architecture = elf.elf.architecture();

    let program_translator = map_err(raptor::translator::ProgramTranslator::new(
        architecture.box_clone(),
        architecture.calling_convention(),
        &elf.elf,
    ))?;

    let functions = translate(program_translator.function_translator(), program)?;

    let mut program = raptor::ir::Program::new();
    for function in functions {
        let index = function.index().unwrap();
        program.replace_function(index, function);
    }

    Ok(ir::Program { program })
}

#[pymodule]
#[pyo3(name = "ir")]
fn raptor_ir_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ir::Block>()?;
    m.add_class::<ir::Constant>()?;
    m.add_class::<ir::ControlFlowGraph>()?;
    m.add_class::<ir::Dereference>()?;
    m.add_class::<ir::Edge>()?;
    m.add_class::<ir::Expression>()?;
    m.add_class::<ir::Function>()?;
    m.add_class::<ir::FunctionLocation>()?;
    m.add_class::<ir::Instruction>()?;
    m.add_class::<ir::LValue>()?;
    m.add_class::<ir::Operation>()?;
    m.add_class::<ir::Program>()?;
    m.add_class::<ir::ProgramLocation>()?;
    m.add_class::<ir::Reference>()?;
    m.add_class::<ir::RValue>()?;
    m.add_class::<ir::Scalar>()?;
    m.add_class::<ir::StackVariable>()?;
    m.add_class::<ir::Variable>()?;
    Ok(())
}

#[pymodule]
#[pyo3(name = "raptor")]
pub fn raptor_module(py: Python, m: &PyModule) -> PyResult<()> {
    use analysis::PyInit_raptor_analysis;

    m.add_wrapped(wrap_pymodule!(ir))?;
    m.add("analysis", wrap_pymodule!(raptor_analysis)(py))?;
    m.add_wrapped(wrap_pyfunction!(translate_function))?;
    m.add_wrapped(wrap_pyfunction!(translate_program_parallel))?;
    Ok(())
}
