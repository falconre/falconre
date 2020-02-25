use crate::falcon::il;
use crate::map_err;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::collections::HashMap;

mod constants;
mod location_set;

pub use constants::Constants;
pub use location_set::LocationSet;

#[pyfunction]
pub(crate) fn def_use(
    function: &il::Function,
) -> PyResult<HashMap<il::ProgramLocation, LocationSet>> {
    map_err(falcon::analysis::def_use(&function.function)).map(|rd| {
        rd.into_iter()
            .map(|(pl, ls)| (pl.into(), ls.into()))
            .collect()
    })
}

#[pyfunction]
pub(crate) fn reaching_definitions(
    function: &il::Function,
) -> PyResult<HashMap<il::ProgramLocation, LocationSet>> {
    map_err(falcon::analysis::reaching_definitions(&function.function)).map(|rd| {
        rd.into_iter()
            .map(|(pl, ls)| (pl.into(), ls.into()))
            .collect()
    })
}

#[pyfunction]
pub(crate) fn use_def(
    function: &il::Function,
) -> PyResult<HashMap<il::ProgramLocation, LocationSet>> {
    map_err(falcon::analysis::def_use(&function.function)).map(|rd| {
        rd.into_iter()
            .map(|(pl, ls)| (pl.into(), ls.into()))
            .collect()
    })
}

#[pyfunction]
pub fn constants(function: &il::Function) -> PyResult<HashMap<il::ProgramLocation, Constants>> {
    map_err(falcon::analysis::constants::constants(&function.function)).map(|rd| {
        rd.into_iter()
            .map(|(pl, cs)| (pl.into(), cs.into()))
            .collect()
    })
}

#[pymodule(analysis)]
fn falcon_analysis_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Constants>()?;
    m.add_class::<LocationSet>()?;
    m.add_wrapped(wrap_pyfunction!(constants))?;
    m.add_wrapped(wrap_pyfunction!(def_use))?;
    m.add_wrapped(wrap_pyfunction!(reaching_definitions))?;
    m.add_wrapped(wrap_pyfunction!(use_def))?;
    Ok(())
}
