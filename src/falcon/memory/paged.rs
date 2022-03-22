use crate::map_err;
use pyo3::prelude::*;

use super::MemoryPermissions;
use crate::falcon::architecture::Endian;
use crate::falcon::il::{Constant, Expression};

pub struct Memory<V: falcon::memory::Value> {
    memory: falcon::memory::paged::Memory<V>,
}

impl<V: falcon::memory::Value> Memory<V> {
    fn new(endian: &Endian) -> Memory<V> {
        Memory {
            memory: falcon::memory::paged::Memory::<V>::new(endian.endian.clone()),
        }
    }

    fn endian(&self) -> Endian {
        self.memory.endian().into()
    }

    fn permissions(&self, address: u64) -> Option<MemoryPermissions> {
        self.memory.permissions(address).map(|m| m.into())
    }

    fn store(&mut self, address: u64, value: V) -> PyResult<()> {
        map_err(self.memory.store(address, value))
    }

    fn load(&self, address: u64, bits: usize) -> PyResult<Option<V>> {
        map_err(self.memory.load(address, bits))
    }
}

#[pyclass]
pub struct ConstantMemory {
    memory: Memory<falcon::il::Constant>,
}

#[pymethods]
impl ConstantMemory {
    #[new]
    fn new(endian: &Endian) -> ConstantMemory {
        ConstantMemory {
            memory: Memory::<falcon::il::Constant>::new(endian),
        }
    }

    #[getter(endian)]
    fn endian(&self) -> Endian {
        self.memory.endian()
    }

    fn permissions(&self, address: u64) -> Option<MemoryPermissions> {
        self.memory.permissions(address)
    }

    fn store(&mut self, address: u64, constant: &Constant) -> PyResult<()> {
        self.memory.store(address, constant.constant.clone())
    }

    fn load(&self, address: u64, bits: usize) -> PyResult<Option<Constant>> {
        self.memory
            .load(address, bits)
            .map(|option| option.map(|constant| constant.into()))
    }
}

#[pyclass]
pub struct ExpressionMemory {
    memory: Memory<falcon::il::Expression>,
}

#[pymethods]
impl ExpressionMemory {
    #[new]
    fn new(endian: &Endian) -> ExpressionMemory {
        ExpressionMemory {
            memory: Memory::new(endian),
        }
    }

    #[getter(endian)]
    fn endian(&self) -> Endian {
        self.memory.endian()
    }

    fn permissions(&self, address: u64) -> Option<MemoryPermissions> {
        self.memory.permissions(address)
    }

    fn store(&mut self, address: u64, expression: &Expression) -> PyResult<()> {
        self.memory.store(address, expression.expression.clone())
    }

    fn load(&self, address: u64, bits: usize) -> PyResult<Option<Expression>> {
        self.memory
            .load(address, bits)
            .map(|option| option.map(|expression| expression.into()))
    }
}
