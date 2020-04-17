use crate::map_err;
use crate::raptor::ir;
use pyo3::class::PyObjectProtocol;
use pyo3::prelude::*;

#[pyclass]
#[derive(Eq, Hash, PartialEq)]
pub struct Expression {
    pub(crate) expression: raptor::ir::Expression<raptor::ir::Constant>,
}

#[pymethods]
impl Expression {
    fn contains_reference(&self) -> bool {
        self.expression.contains_reference()
    }

    fn bits(&self) -> usize {
        self.expression.bits()
    }

    fn variables(&self) -> Vec<ir::Variable> {
        self.expression
            .variables()
            .into_iter()
            .map(|variable| variable.clone().into())
            .collect()
    }

    fn variable(&self) -> Option<ir::Variable> {
        self.expression
            .variable()
            .map(|variable| variable.clone().into())
    }

    fn stack_pointer(&self) -> Option<ir::StackVariable> {
        self.expression
            .stack_pointer()
            .map(|stack_variable| stack_variable.clone().into())
    }

    fn lhs(&self) -> Option<Expression> {
        match &self.expression {
            raptor::ir::Expression::Add(lhs, _)
            | raptor::ir::Expression::Sub(lhs, _)
            | raptor::ir::Expression::Mul(lhs, _)
            | raptor::ir::Expression::Divu(lhs, _)
            | raptor::ir::Expression::Modu(lhs, _)
            | raptor::ir::Expression::Divs(lhs, _)
            | raptor::ir::Expression::Mods(lhs, _)
            | raptor::ir::Expression::And(lhs, _)
            | raptor::ir::Expression::Or(lhs, _)
            | raptor::ir::Expression::Xor(lhs, _)
            | raptor::ir::Expression::Shl(lhs, _)
            | raptor::ir::Expression::Shr(lhs, _)
            | raptor::ir::Expression::Cmpeq(lhs, _)
            | raptor::ir::Expression::Cmpneq(lhs, _)
            | raptor::ir::Expression::Cmpltu(lhs, _)
            | raptor::ir::Expression::Cmplts(lhs, _) => Some(Expression {
                expression: lhs.as_ref().clone().into(),
            }),
            raptor::ir::Expression::LValue(_)
            | raptor::ir::Expression::RValue(_)
            | raptor::ir::Expression::Trun(_, _)
            | raptor::ir::Expression::Sext(_, _)
            | raptor::ir::Expression::Zext(_, _)
            | raptor::ir::Expression::Ite(_, _, _) => None,
        }
    }

    fn rhs(&self) -> Option<Expression> {
        match &self.expression {
            raptor::ir::Expression::Add(_, rhs)
            | raptor::ir::Expression::Sub(_, rhs)
            | raptor::ir::Expression::Mul(_, rhs)
            | raptor::ir::Expression::Divu(_, rhs)
            | raptor::ir::Expression::Modu(_, rhs)
            | raptor::ir::Expression::Divs(_, rhs)
            | raptor::ir::Expression::Mods(_, rhs)
            | raptor::ir::Expression::And(_, rhs)
            | raptor::ir::Expression::Or(_, rhs)
            | raptor::ir::Expression::Xor(_, rhs)
            | raptor::ir::Expression::Shl(_, rhs)
            | raptor::ir::Expression::Shr(_, rhs)
            | raptor::ir::Expression::Cmpeq(_, rhs)
            | raptor::ir::Expression::Cmpneq(_, rhs)
            | raptor::ir::Expression::Cmpltu(_, rhs)
            | raptor::ir::Expression::Cmplts(_, rhs) => Some(Expression {
                expression: rhs.as_ref().clone().into(),
            }),
            raptor::ir::Expression::LValue(_)
            | raptor::ir::Expression::RValue(_)
            | raptor::ir::Expression::Trun(_, _)
            | raptor::ir::Expression::Sext(_, _)
            | raptor::ir::Expression::Zext(_, _)
            | raptor::ir::Expression::Ite(_, _, _) => None,
        }
    }

    fn json(&self) -> PyResult<String> {
        map_err(serde_json::to_string(&self.expression))
    }
}

#[pyproto]
impl<'p> PyObjectProtocol<'p> for Expression {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.expression.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.expression.to_string())
    }

    fn __hash__(&self) -> PyResult<isize> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        Ok(s.finish() as isize)
    }
}

impl From<raptor::ir::Expression<raptor::ir::Constant>> for Expression {
    fn from(expression: raptor::ir::Expression<raptor::ir::Constant>) -> Expression {
        Expression { expression }
    }
}
