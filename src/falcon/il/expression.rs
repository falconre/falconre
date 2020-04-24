use super::{Constant, Scalar};
use crate::map_err;
use pyo3::class::PyObjectProtocol;
use pyo3::prelude::*;

#[pyclass]
pub struct Expression {
    pub(crate) expression: falcon::il::Expression,
}

fn binop<F>(lhs: &Expression, rhs: &Expression, f: F) -> PyResult<Expression>
where
    F: Fn(
        falcon::il::Expression,
        falcon::il::Expression,
    ) -> Result<falcon::il::Expression, falcon::error::Error>,
{
    Ok(Expression {
        expression: map_err(f(lhs.expression.clone(), rhs.expression.clone()))?,
    })
}

#[pymethods]
impl Expression {
    fn bits(&self) -> usize {
        self.expression.bits()
    }

    fn json(&self) -> PyResult<String> {
        map_err(serde_json::to_string(&self.expression))
    }

    fn get_constant(&self) -> Option<Constant> {
        self.expression.get_constant().map(|c| Constant {
            constant: c.clone(),
        })
    }

    fn scalars(&self) -> Vec<Scalar> {
        self.expression
            .scalars()
            .into_iter()
            .map(|s| Scalar { scalar: s.clone() })
            .collect()
    }

    fn eval(&self) -> PyResult<Constant> {
        Ok(Constant {
            constant: map_err(falcon::executor::eval(&self.expression))?,
        })
    }

    #[staticmethod]
    fn add(lhs: &Expression, rhs: &Expression) -> PyResult<Expression> {
        binop(lhs, rhs, falcon::il::Expression::add)
    }

    #[staticmethod]
    fn sub(lhs: &Expression, rhs: &Expression) -> PyResult<Expression> {
        binop(lhs, rhs, falcon::il::Expression::sub)
    }

    #[staticmethod]
    fn mul(lhs: &Expression, rhs: &Expression) -> PyResult<Expression> {
        binop(lhs, rhs, falcon::il::Expression::mul)
    }

    #[staticmethod]
    fn divu(lhs: &Expression, rhs: &Expression) -> PyResult<Expression> {
        binop(lhs, rhs, falcon::il::Expression::divu)
    }

    #[staticmethod]
    fn modu(lhs: &Expression, rhs: &Expression) -> PyResult<Expression> {
        binop(lhs, rhs, falcon::il::Expression::modu)
    }

    #[staticmethod]
    fn divs(lhs: &Expression, rhs: &Expression) -> PyResult<Expression> {
        binop(lhs, rhs, falcon::il::Expression::divs)
    }

    #[staticmethod]
    fn mods(lhs: &Expression, rhs: &Expression) -> PyResult<Expression> {
        binop(lhs, rhs, falcon::il::Expression::mods)
    }

    #[staticmethod]
    fn and(lhs: &Expression, rhs: &Expression) -> PyResult<Expression> {
        binop(lhs, rhs, falcon::il::Expression::and)
    }

    #[staticmethod]
    fn or(lhs: &Expression, rhs: &Expression) -> PyResult<Expression> {
        binop(lhs, rhs, falcon::il::Expression::or)
    }

    #[staticmethod]
    fn xor(lhs: &Expression, rhs: &Expression) -> PyResult<Expression> {
        binop(lhs, rhs, falcon::il::Expression::xor)
    }

    #[staticmethod]
    fn cmpeq(lhs: &Expression, rhs: &Expression) -> PyResult<Expression> {
        binop(lhs, rhs, falcon::il::Expression::cmpeq)
    }

    #[staticmethod]
    fn cmpneq(lhs: &Expression, rhs: &Expression) -> PyResult<Expression> {
        binop(lhs, rhs, falcon::il::Expression::cmpneq)
    }

    #[staticmethod]
    fn cmplts(lhs: &Expression, rhs: &Expression) -> PyResult<Expression> {
        binop(lhs, rhs, falcon::il::Expression::cmplts)
    }

    #[staticmethod]
    fn cmpltu(lhs: &Expression, rhs: &Expression) -> PyResult<Expression> {
        binop(lhs, rhs, falcon::il::Expression::cmpltu)
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
}

impl From<falcon::il::Expression> for Expression {
    fn from(expression: falcon::il::Expression) -> Expression {
        Expression { expression }
    }
}
