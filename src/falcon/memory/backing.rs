use pyo3::prelude::*;
use std::collections::BTreeMap;

use super::MemoryPermissions;
use crate::falcon::architecture::Endian;

#[pyclass]
pub struct Section {
    section: falcon::memory::backing::Section,
}

#[pymethods]
impl Section {
    fn data(&self) -> &[u8] {
        self.section.data()
    }

    fn permissions(&self) -> MemoryPermissions {
        self.section.permissions().into()
    }
}

impl From<falcon::memory::backing::Section> for Section {
    fn from(section: falcon::memory::backing::Section) -> Section {
        Section { section }
    }
}

#[pyclass]
pub struct Memory {
    pub(crate) memory: falcon::memory::backing::Memory,
}

#[pymethods]
impl Memory {
    #[new]
    fn new(endian: &Endian) -> Memory {
        Memory {
            memory: falcon::memory::backing::Memory::new(endian.endian.clone()),
        }
    }

    fn sections(&self) -> BTreeMap<u64, Section> {
        self.memory
            .sections()
            .iter()
            .map(|(u, s)| (*u, s.clone().into()))
            .collect()
    }

    fn get8(&self, address: u64) -> Option<u8> {
        self.memory.get8(address)
    }

    fn set_memory(&mut self, address: u64, data: Vec<u8>, permissions: &MemoryPermissions) {
        self.memory
            .set_memory(address, data, permissions.memory_permissions)
    }
}

impl From<falcon::memory::backing::Memory> for Memory {
    fn from(memory: falcon::memory::backing::Memory) -> Memory {
        Memory { memory }
    }
}
