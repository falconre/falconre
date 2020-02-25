use pyo3::class::PyObjectProtocol;
use pyo3::prelude::*;

#[pyclass]
pub struct Endian {
    pub endian: falcon::architecture::Endian,
}

#[pymethods]
impl Endian {
    #[staticmethod]
    fn little() -> Endian {
        Endian {
            endian: falcon::architecture::Endian::Little,
        }
    }

    #[staticmethod]
    fn big() -> Endian {
        Endian {
            endian: falcon::architecture::Endian::Big,
        }
    }

    fn str(&self) -> &str {
        match self.endian {
            falcon::architecture::Endian::Little => "little",
            falcon::architecture::Endian::Big => "big",
        }
    }
}

#[pyproto]
impl<'p> PyObjectProtocol<'p> for Endian {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.str().to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.str().to_string())
    }
}

impl From<falcon::architecture::Endian> for Endian {
    fn from(endian: falcon::architecture::Endian) -> Endian {
        Endian { endian }
    }
}

#[pyclass]
pub struct Architecture {
    pub architecture: Box<dyn falcon::architecture::Architecture>,
}

#[pymethods]
impl Architecture {
    fn name(&self) -> &str {
        self.architecture.name()
    }

    fn endian(&self) -> Endian {
        Endian {
            endian: self.architecture.endian(),
        }
    }

    #[staticmethod]
    fn amd64() -> Architecture {
        Architecture {
            architecture: Box::new(falcon::architecture::Amd64::new()),
        }
    }

    #[staticmethod]
    fn mips() -> Architecture {
        Architecture {
            architecture: Box::new(falcon::architecture::Mips::new()),
        }
    }

    #[staticmethod]
    fn mipsel() -> Architecture {
        Architecture {
            architecture: Box::new(falcon::architecture::Mipsel::new()),
        }
    }

    #[staticmethod]
    fn x86() -> Architecture {
        Architecture {
            architecture: Box::new(falcon::architecture::X86::new()),
        }
    }
}
