use crate::falcon::il;
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct LocationSet {
    pub(crate) location_set: falcon::analysis::LocationSet,
}

#[pymethods]
impl LocationSet {
    #[new]
    fn new() -> LocationSet {
        LocationSet {
            location_set: falcon::analysis::LocationSet::new(),
        }
    }

    fn contains(&self, location: &il::ProgramLocation) -> bool {
        self.location_set.contains(&location.program_location)
    }

    fn insert(&mut self, location: &il::ProgramLocation) {
        self.location_set.insert(location.program_location.clone())
    }

    #[getter(locations)]
    fn locations(&self) -> Vec<il::ProgramLocation> {
        self.location_set
            .locations()
            .iter()
            .map(|pl| pl.clone().into())
            .collect()
    }
}

impl From<falcon::analysis::LocationSet> for LocationSet {
    fn from(location_set: falcon::analysis::LocationSet) -> LocationSet {
        LocationSet { location_set }
    }
}
