use pyo3::prelude::*;

mod driver;
mod memory;
mod state;

pub use driver::Driver;
pub use memory::Memory;
pub use state::State;

#[pyclass(unsendable)]
pub struct Successor {
    successor: finch::executor::Successor,
}

#[pymethods]
impl Successor {
    #[getter(type)]
    fn type_(&self) -> String {
        match self.successor.type_() {
            finch::executor::SuccessorType::FallThrough => "FallThrough",
            finch::executor::SuccessorType::Branch(_) => "Branch",
            finch::executor::SuccessorType::Raise(_) => "Raise",
        }
        .to_string()
    }

    #[getter(branch_target)]
    fn branch_target(&self) -> Option<u64> {
        match self.successor.type_() {
            finch::executor::SuccessorType::Branch(target) => Some(*target),
            _ => None,
        }
    }

    #[getter(state)]
    fn state(&self) -> State {
        State {
            state: self.successor.state().clone(),
        }
    }
}

impl From<finch::executor::Successor> for Successor {
    fn from(successor: finch::executor::Successor) -> Successor {
        Successor { successor }
    }
}
