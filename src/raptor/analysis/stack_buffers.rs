use crate::map_err;
use crate::raptor::ir;
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct StackBuffers {
    pub(crate) stack_buffers: raptor::analysis::stack_buffers::StackBuffers,
}

#[pymethods]
impl StackBuffers {
    fn eval(&self, expression: &ir::Expression) -> PyResult<Option<ir::StackVariable>> {
        map_err(self.stack_buffers.eval(&expression.expression)).map(|lattice_stack_buffer| {
            lattice_stack_buffer
                .value()
                .map(|stack_buffer| ir::StackVariable {
                    stack_variable: stack_buffer.stack_variable().clone(),
                })
        })
    }
}

impl From<raptor::analysis::stack_buffers::StackBuffers> for StackBuffers {
    fn from(stack_buffers: raptor::analysis::stack_buffers::StackBuffers) -> StackBuffers {
        StackBuffers { stack_buffers }
    }
}
