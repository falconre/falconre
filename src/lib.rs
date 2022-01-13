use pyo3::prelude::*;
use pyo3::wrap_pymodule;

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
fn falconre(_py: Python, m: &PyModule) -> PyResult<()> {
    use crate::falcon::PyInit_falcon;
    use crate::finch::PyInit_finch;
    // use crate::raptor::PyInit_raptor;

    m.add_wrapped(wrap_pymodule!(falcon))?;
    m.add_wrapped(wrap_pymodule!(finch))?;
    // m.add_wrapped(wrap_pymodule!(raptor))?;
    Ok(())
}
