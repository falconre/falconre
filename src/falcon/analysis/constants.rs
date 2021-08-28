use crate::falcon::il;
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct Constants {
    pub(crate) constants: falcon::analysis::constants::Constants,
}

#[pymethods]
impl Constants {
    fn scalar(&self, scalar: &il::Scalar) -> Option<il::Constant> {
        self.constants
            .scalar(&scalar.scalar)
            .map(|constant| constant.clone().into())
    }

    fn eval(&self, expression: &il::Expression) -> Option<il::Constant> {
        self.constants
            .eval(&expression.expression)
            .map(|constant| constant.into())
    }
}

impl From<falcon::analysis::constants::Constants> for Constants {
    fn from(constants: falcon::analysis::constants::Constants) -> Constants {
        Constants {
            constants,
        }
    }
}
