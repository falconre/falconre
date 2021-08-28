use super::{Memory, Successor};
use crate::falcon::il;
use crate::map_err;
use pyo3::prelude::*;

#[pyclass(unsendable)]
pub struct State {
    pub(crate) state: finch::executor::State,
}

#[pymethods]
impl State {
    #[new]
    fn new(memory: &Memory) -> State {
        State {
            state: finch::executor::State::new(
                memory.memory.clone(),
                Box::new(finch::platform::Dummy::new()),
            ),
        }
    }

    #[getter(memory)]
    fn memory(&self) -> Memory {
        self.state.memory().clone().into()
    }

    fn set_scalar(&mut self, name: &str, value: &il::Expression) -> PyResult<()> {
        map_err(self.state.set_scalar(name, &value.expression))
    }

    fn scalar(&self, name: &str) -> Option<il::Expression> {
        self.state.scalar(name).map(|s| s.into())
    }

    #[getter(scalars)]
    fn scalars(&self) -> Vec<String> {
        self.state.scalars()
    }

    fn symbolize_expression(&self, expression: &il::Expression) -> PyResult<il::Expression> {
        map_err(self.state.symbolize_expression(&expression.expression)).map(|expr| expr.into())
    }

    fn eval(&self, expression: &il::Expression) -> PyResult<Option<il::Constant>> {
        map_err(self.state.eval(&expression.expression))
            .map(|option_constant| option_constant.map(|constant| constant.into()))
    }

    fn symbolize_and_eval(&self, expression: &il::Expression) -> PyResult<Option<il::Constant>> {
        map_err(self.state.symbolize_and_eval(&expression.expression))
            .map(|option_constant| option_constant.map(|constant| constant.into()))
    }

    fn execute(&self, operation: &il::Operation) -> PyResult<Vec<Successor>> {
        map_err(self.state.clone().execute(&operation.operation)).map(|successors| {
            successors
                .into_iter()
                .map(|successor| successor.into())
                .collect()
        })
    }

    fn make_symbolic_buffer(
        &mut self,
        name: &str,
        length: usize,
    ) -> PyResult<(u64, Vec<il::Expression>)> {
        map_err(self.state.make_symbolic_buffer(name, length))
            .map(|(addr, vars)| (addr, vars.into_iter().map(|v| v.into()).collect()))
    }

    fn make_symbolic_string(&mut self, name: &str, length: usize) -> PyResult<u64> {
        map_err(self.state.make_symbolic_string(name, length))
    }

    fn get_string(&self, address: u64) -> PyResult<Option<String>> {
        map_err(self.state.get_string(address))
    }

    fn store(&mut self, address: u64, expr: &il::Expression) -> PyResult<()> {
        map_err(self.state.memory_mut().store(address, &expr.expression))
    }
}
