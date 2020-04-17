use crate::map_err;
use pyo3::prelude::*;

use crate::falcon::architecture::Endian;
use crate::falcon::il;

#[pyclass]
pub struct Memory {
    pub memory: finch::executor::Memory,
}

#[pymethods]
impl Memory {
    #[new]
    fn new(endian: &Endian) -> Memory {
        let endian: falcon::architecture::Endian = endian.endian.clone();
        Memory {
            memory: finch::executor::Memory::new(endian),
        }
    }

    #[staticmethod]
    fn new_with_backing(
        endian: &Endian,
        backing: &crate::falcon::memory::backing::Memory,
    ) -> Memory {
        Memory {
            memory: finch::executor::Memory::new_with_backing(
                endian.endian.clone(),
                falcon::RC::new(backing.memory.clone()),
            ),
        }
    }

    fn store(&mut self, address: u64, value: &il::Expression) -> PyResult<()> {
        map_err(self.memory.store(address, &value.expression))
    }

    fn load(&self, address: u64, bits: usize) -> PyResult<Option<il::Expression>> {
        map_err(self.memory.load(address, bits))
            .map(|option_expr| option_expr.map(|expr| expr.into()))
    }
}

impl From<finch::executor::Memory> for Memory {
    fn from(memory: finch::executor::Memory) -> Memory {
        Memory { memory }
    }
}
