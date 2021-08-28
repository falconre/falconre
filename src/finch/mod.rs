pub mod executor;

use crate::map_err;
use pyo3::prelude::*;
use pyo3::{wrap_pyfunction, wrap_pymodule};

#[pyfunction]
fn amd64_standard_load(filename: &str, base_path: Option<String>) -> PyResult<executor::Driver> {
    let base_path = base_path.map(|bp| bp.into());
    let driver = map_err(finch::platform::linux::Amd64::standard_load(
        filename, base_path,
    ))?;
    Ok(executor::Driver { driver: driver })
}

#[pymodule]
#[pyo3(name = "executor")]
fn finch_executor_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<executor::Driver>()?;
    m.add_class::<executor::Memory>()?;
    m.add_class::<executor::State>()?;
    m.add_class::<executor::Successor>()?;
    Ok(())
}

#[pymodule]
#[pyo3(name = "finch")]
pub fn finch_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(executor))?;
    m.add_wrapped(wrap_pyfunction!(amd64_standard_load))?;
    Ok(())
}
