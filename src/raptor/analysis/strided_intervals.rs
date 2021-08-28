use crate::map_err;
use crate::raptor::ir;
use pyo3::class::PyObjectProtocol;
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct StridedIntervals {
    pub(crate) state: raptor::analysis::strided_intervals::State,
}

#[pymethods]
impl StridedIntervals {
    fn variable(&self, variable: &ir::Variable) -> Option<StridedInterval> {
        self.state
            .variable(&variable.variable)
            .map(|strided_interval| strided_interval.clone().into())
    }

    fn eval(&self, expression: &ir::Expression) -> PyResult<StridedInterval> {
        map_err(self.state.eval(&(&expression.expression).into()))
            .map(|strided_interval| strided_interval.into())
    }
}

#[pyclass]
#[derive(Clone)]
pub struct StridedInterval {
    pub(crate) strided_interval: raptor::analysis::strided_intervals::StridedInterval,
}

impl From<raptor::analysis::strided_intervals::State> for StridedIntervals {
    fn from(state: raptor::analysis::strided_intervals::State) -> StridedIntervals {
        StridedIntervals { state }
    }
}

#[pymethods]
impl StridedInterval {
    fn stride(&self) -> usize {
        self.strided_interval.stride()
    }

    fn lo(&self) -> Option<ir::Constant> {
        self.strided_interval
            .interval()
            .lo()
            .value()
            .map(|constant| constant.clone().into())
    }

    fn hi(&self) -> Option<ir::Constant> {
        self.strided_interval
            .interval()
            .hi()
            .value()
            .map(|constant| constant.clone().into())
    }
}

#[pyproto]
impl<'p> PyObjectProtocol<'p> for StridedInterval {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.strided_interval.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.strided_interval.to_string())
    }
}

impl From<raptor::analysis::strided_intervals::StridedInterval> for StridedInterval {
    fn from(
        strided_interval: raptor::analysis::strided_intervals::StridedInterval,
    ) -> StridedInterval {
        StridedInterval {
            strided_interval,
        }
    }
}
