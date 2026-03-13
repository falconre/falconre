use pyo3::prelude::*;

pub mod falcon;
// pub mod finch;
// pub mod raptor;

pub fn map_err<V, E: std::error::Error>(r: Result<V, E>) -> PyResult<V> {
    Ok(r.map_err(|e| pyo3::exceptions::PyException::new_err(format!("{}", e)))?)
}

#[pymodule]
#[pyo3(name = "falconre")]
fn falconre(m: &Bound<'_, PyModule>) -> PyResult<()> {
    falcon::register_falcon(m)?;
    // finch::register_finch(m)?;
    // raptor::register_raptor(m)?;
    Ok(())
}
