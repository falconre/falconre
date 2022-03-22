pub mod executor;

use crate::map_err;
use pyo3::prelude::*;

#[pyfunction]
fn amd64_standard_load(filename: &str, base_path: Option<String>) -> PyResult<executor::Driver> {
    let base_path = base_path.map(|bp| bp.into());
    let driver = map_err(finch::platform::linux::Amd64::standard_load(
        filename, base_path,
    ))?;
    Ok(executor::Driver { driver })
}

#[pymodule]
#[pyo3(name = "finch")]
pub fn register_finch_module(py: Python, parent_module: &PyModule) -> PyResult<()> {
    let m = PyModule::new(py, "finch")?;

    m.add_class::<executor::Driver>()?;
    m.add_class::<executor::Memory>()?;
    m.add_class::<executor::State>()?;
    m.add_class::<executor::Successor>()?;
    m.add_function(wrap_pyfunction!(amd64_standard_load, m)?)?;

    parent_module.add_submodule(m)?;

    Ok(())
}
