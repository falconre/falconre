use crate::str_err;
use pyo3::prelude::*;

pub mod backing;
pub mod paged;

#[pyclass]
pub struct MemoryPermissions {
    pub memory_permissions: falcon::memory::MemoryPermissions,
}

#[pymethods]
impl MemoryPermissions {
    #[new]
    fn new(bits: u32) -> PyResult<MemoryPermissions> {
        let memory_permissions = falcon::memory::MemoryPermissions::from_bits(bits)
            .ok_or(str_err("Invalid bits for MemoryPermissions"))?;
        Ok(MemoryPermissions {
            memory_permissions: memory_permissions,
        })
    }

    fn bits(&self) -> u32 {
        self.memory_permissions.bits()
    }
}

impl From<falcon::memory::MemoryPermissions> for MemoryPermissions {
    fn from(memory_permissions: falcon::memory::MemoryPermissions) -> MemoryPermissions {
        MemoryPermissions { memory_permissions }
    }
}
