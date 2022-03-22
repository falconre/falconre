use pyo3::prelude::*;

pub mod falcon;
pub mod finch;
// pub mod raptor;

pub fn map_err<V, E: std::error::Error>(r: Result<V, E>) -> PyResult<V> {
    r.map_err(|e| pyo3::exceptions::PyException::new_err(format!("{}", e)))
}

pub fn str_err<S: Into<String>>(s: S) -> pyo3::PyErr {
    pyo3::exceptions::PyException::new_err(s.into())
}

#[pymodule]
#[pyo3(name = "falconre")]
fn falconre(py: Python, m: &PyModule) -> PyResult<()> {
    falcon::register_falcon_module(py, m)?;
    finch::register_finch_module(py, m)?;

    Ok(())
}
