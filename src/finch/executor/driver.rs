use super::State;
use crate::falcon::architecture::Architecture;
use crate::falcon::il;
use crate::map_err;
use pyo3::prelude::*;

#[pyclass(unsendable)]
pub struct Driver {
    pub(crate) driver: finch::executor::Driver,
}

#[pymethods]
impl Driver {
    #[new]
    fn new(
        program: &il::Program,
        location: &il::ProgramLocation,
        state: &State,
        architecture: &Architecture,
    ) -> Driver {
        Driver {
            driver: finch::executor::Driver::new(
                program.program.clone(),
                location.program_location.clone(),
                state.state.clone(),
                falcon::RC::new(architecture.architecture.box_clone()),
            ),
        }
    }

    fn step(&self) -> PyResult<Vec<Driver>> {
        map_err(self.driver.clone().step())
            .map(|drivers| drivers.into_iter().map(|d| Driver { driver: d }).collect())
    }

    #[getter(location)]
    fn location(&self) -> il::ProgramLocation {
        il::ProgramLocation {
            program_location: self.driver.location().clone(),
        }
    }

    #[getter(address)]
    fn address(&self) -> Option<u64> {
        self.driver.address()
    }

    #[getter(instruction)]
    fn instruction(&self) -> Option<il::Instruction> {
        self.driver
            .instruction()
            .map(|instruction| il::Instruction { instruction })
    }

    #[getter(state)]
    fn state(&self) -> State {
        State {
            state: self.driver.state().clone(),
        }
    }

    fn set_state(&mut self, state: &State) {
        *self.driver.state_mut() = state.state.clone();
    }
}
