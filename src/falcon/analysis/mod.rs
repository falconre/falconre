use crate::falcon::il;
use crate::map_err;
use pyo3::prelude::*;
use std::collections::HashMap;

mod constants_;
mod location_set;

pub use constants_::Constants;
pub use location_set::LocationSet;

#[pyfunction]
fn def_use(function: &il::Function) -> PyResult<HashMap<il::ProgramLocation, LocationSet>> {
    map_err(falcon::analysis::def_use(&function.function)).map(|rd| {
        rd.into_iter()
            .map(|(pl, ls)| (pl.into(), ls.into()))
            .collect()
    })
}

#[pyfunction]
fn reaching_definitions(
    function: &il::Function,
) -> PyResult<HashMap<il::ProgramLocation, LocationSet>> {
    map_err(falcon::analysis::reaching_definitions(&function.function)).map(|rd| {
        rd.into_iter()
            .map(|(pl, ls)| (pl.into(), ls.into()))
            .collect()
    })
}

#[pyfunction]
fn use_def(function: &il::Function) -> PyResult<HashMap<il::ProgramLocation, LocationSet>> {
    map_err(falcon::analysis::use_def(&function.function)).map(|rd| {
        rd.into_iter()
            .map(|(pl, ls)| (pl.into(), ls.into()))
            .collect()
    })
}

#[pyfunction]
fn constants(function: &il::Function) -> PyResult<HashMap<il::ProgramLocation, Constants>> {
    map_err(falcon::analysis::constants::constants(&function.function)).map(|rd| {
        rd.into_iter()
            .map(|(pl, cs)| (pl.into(), cs.into()))
            .collect()
    })
}

#[pyfunction]
fn test_function(x: usize, y: usize) -> usize {
    x + y
}

pub(crate) fn register_analysis_module(py: Python, parent_module: &PyModule) -> PyResult<()> {
    let m = PyModule::new(py, "analysis")?;
    m.add_class::<Constants>()?;
    m.add_class::<LocationSet>()?;
    m.add_function(wrap_pyfunction!(constants, m)?)?;
    m.add_function(wrap_pyfunction!(def_use, m)?)?;
    m.add_function(wrap_pyfunction!(reaching_definitions, m)?)?;
    m.add_function(wrap_pyfunction!(use_def, m)?)?;
    m.add_function(wrap_pyfunction!(test_function, m)?)?;
    parent_module.add_submodule(m)?;
    Ok(())
}
