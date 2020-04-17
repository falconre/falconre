use crate::raptor::ir;
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct LocationSet {
    pub(crate) location_set: raptor::analysis::LocationSet,
}

#[pymethods]
impl LocationSet {
    #[new]
    fn new() -> LocationSet {
        LocationSet {
            location_set: raptor::analysis::LocationSet::new(),
        }
    }

    fn contains(&self, location: &ir::ProgramLocation) -> bool {
        self.location_set.contains(&location.program_location)
    }

    fn insert(&mut self, location: &ir::ProgramLocation) {
        self.location_set.insert(location.program_location.clone())
    }

    fn locations(&self) -> Vec<ir::ProgramLocation> {
        self.location_set
            .locations()
            .into_iter()
            .map(|pl| pl.clone().into())
            .collect()
    }
}

impl From<raptor::analysis::LocationSet> for LocationSet {
    fn from(location_set: raptor::analysis::LocationSet) -> LocationSet {
        LocationSet {
            location_set: location_set,
        }
    }
}
