use crate::raptor::ir;
use crate::map_err;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;


#[pyfunction]
pub(crate) fn dead_code_elimination(
    function: &ir::Function,
) -> PyResult<ir::Function> {
    map_err(
        raptor::analysis::dead_code_elimination(&function.function)
    ).map(|function| function.into())
}


#[pymodule(raptor_analysis)]
fn falcon_analysis_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(dead_code_elimination))?;
    Ok(())
}