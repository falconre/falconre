use crate::map_err;
use crate::raptor::ir;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::collections::HashMap;

mod location_set;
mod stack_buffers;
mod strided_intervals;

pub use location_set::LocationSet;
pub use stack_buffers::StackBuffers;
pub use strided_intervals::{StridedInterval, StridedIntervals};

#[pyfunction]
pub(crate) fn dead_code_elimination(function: &ir::Function) -> PyResult<ir::Function> {
    map_err(raptor::analysis::dead_code_elimination(&function.function))
        .map(|function| function.into())
}

#[pyfunction]
pub(crate) fn reaching_definitions(
    function: &ir::Function,
) -> PyResult<HashMap<ir::ProgramLocation, LocationSet>> {
    map_err(raptor::analysis::reaching_definitions(&function.function)).map(|rd| {
        rd.into_iter()
            .map(|(pl, ls)| (pl.into(), ls.into()))
            .collect()
    })
}

#[pyfunction]
pub(crate) fn use_def(
    function: &ir::Function,
) -> PyResult<HashMap<ir::ProgramLocation, LocationSet>> {
    map_err(raptor::analysis::use_def(&function.function)).map(|rd| {
        rd.into_iter()
            .map(|(pl, ls)| (pl.into(), ls.into()))
            .collect()
    })
}

#[pyfunction]
pub(crate) fn def_use(
    function: &ir::Function,
) -> PyResult<HashMap<ir::ProgramLocation, LocationSet>> {
    map_err(raptor::analysis::def_use(&function.function)).map(|rd| {
        rd.into_iter()
            .map(|(pl, ls)| (pl.into(), ls.into()))
            .collect()
    })
}

#[pyfunction]
pub(crate) fn variable_use_def(
    function: &ir::Function,
) -> PyResult<HashMap<ir::ProgramLocation, HashMap<ir::Variable, LocationSet>>> {
    map_err(raptor::analysis::variable_use_def(&function.function)).map(|rd| {
        rd.into_iter()
            .map(|(pl, vud)| {
                (
                    pl.into(),
                    vud.into_iter()
                        .map(|(var, ls)| (var.into(), ls.into()))
                        .collect(),
                )
            })
            .collect()
    })
}

#[pyfunction]
pub(crate) fn strided_intervals(
    function: &ir::Function,
) -> PyResult<HashMap<ir::ProgramLocation, StridedIntervals>> {
    let strided_intervals = map_err(raptor::analysis::strided_intervals::strided_intervals(
        &function.function,
    ))?;
    let strided_intervals = strided_intervals
        .into_iter()
        .map(|(program_location, state)| (program_location.into(), state.into()))
        .collect();
    Ok(strided_intervals)
}

#[pyfunction]
pub(crate) fn stack_buffers(
    function: &ir::Function,
) -> PyResult<HashMap<ir::ProgramLocation, StackBuffers>> {
    let stack_buffers = map_err(raptor::analysis::stack_buffers::stack_buffers(
        &function.function,
    ))?;
    let stack_buffers = stack_buffers
        .into_iter()
        .map(|(program_location, stack_buffers)| (program_location.into(), stack_buffers.into()))
        .collect();
    Ok(stack_buffers)
}

#[pymodule(raptor_analysis)]
fn falcon_analysis_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<LocationSet>()?;
    m.add_class::<StackBuffers>()?;
    m.add_wrapped(wrap_pyfunction!(dead_code_elimination))?;
    m.add_wrapped(wrap_pyfunction!(def_use))?;
    m.add_wrapped(wrap_pyfunction!(reaching_definitions))?;
    m.add_wrapped(wrap_pyfunction!(stack_buffers))?;
    m.add_wrapped(wrap_pyfunction!(strided_intervals))?;
    m.add_wrapped(wrap_pyfunction!(use_def))?;
    m.add_wrapped(wrap_pyfunction!(variable_use_def))?;
    Ok(())
}
