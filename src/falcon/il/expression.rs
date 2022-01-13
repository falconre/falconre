use crate::map_err;
use pyo3::class::PyObjectProtocol;
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct Expression {
    pub(crate) expression: falcon::il::Expression,
}

#[pymethods]
impl Expression {
    #[getter(bits)]
    fn bits(&self) -> usize {
        self.expression.bits()
    }

    #[getter(json)]
    fn json(&self) -> PyResult<String> {
        map_err(serde_json::to_string(&self.expression))
    }

    fn add(&self, other: Expression) -> PyResult<Expression> {
        Ok(Expression {
            expression: map_err(falcon::il::Expression::add(
                self.expression.clone(),
                other.expression,
            ))?,
        })
    }

    fn sub(&self, other: Expression) -> PyResult<Expression> {
        Ok(Expression {
            expression: map_err(falcon::il::Expression::sub(
                self.expression.clone(),
                other.expression,
            ))?,
        })
    }

    fn mul(&self, other: Expression) -> PyResult<Expression> {
        Ok(Expression {
            expression: map_err(falcon::il::Expression::mul(
                self.expression.clone(),
                other.expression,
            ))?,
        })
    }

    fn divu(&self, other: Expression) -> PyResult<Expression> {
        Ok(Expression {
            expression: map_err(falcon::il::Expression::divu(
                self.expression.clone(),
                other.expression,
            ))?,
        })
    }

    fn modu(&self, other: Expression) -> PyResult<Expression> {
        Ok(Expression {
            expression: map_err(falcon::il::Expression::modu(
                self.expression.clone(),
                other.expression,
            ))?,
        })
    }

    fn divs(&self, other: Expression) -> PyResult<Expression> {
        Ok(Expression {
            expression: map_err(falcon::il::Expression::divs(
                self.expression.clone(),
                other.expression,
            ))?,
        })
    }

    fn mods(&self, other: Expression) -> PyResult<Expression> {
        Ok(Expression {
            expression: map_err(falcon::il::Expression::mods(
                self.expression.clone(),
                other.expression,
            ))?,
        })
    }

    fn and_(&self, other: Expression) -> PyResult<Expression> {
        Ok(Expression {
            expression: map_err(falcon::il::Expression::and(
                self.expression.clone(),
                other.expression,
            ))?,
        })
    }

    fn or_(&self, other: Expression) -> PyResult<Expression> {
        Ok(Expression {
            expression: map_err(falcon::il::Expression::or(
                self.expression.clone(),
                other.expression,
            ))?,
        })
    }

    fn xor_(&self, other: Expression) -> PyResult<Expression> {
        Ok(Expression {
            expression: map_err(falcon::il::Expression::xor(
                self.expression.clone(),
                other.expression,
            ))?,
        })
    }

    fn shl(&self, other: Expression) -> PyResult<Expression> {
        Ok(Expression {
            expression: map_err(falcon::il::Expression::shl(
                self.expression.clone(),
                other.expression,
            ))?,
        })
    }

    fn shr(&self, other: Expression) -> PyResult<Expression> {
        Ok(Expression {
            expression: map_err(falcon::il::Expression::shr(
                self.expression.clone(),
                other.expression,
            ))?,
        })
    }

    fn ashr(&self, other: Expression) -> PyResult<Expression> {
        Ok(Expression {
            expression: map_err(falcon::il::Expression::ashr(
                self.expression.clone(),
                other.expression,
            ))?,
        })
    }

    fn cmpeq(&self, other: Expression) -> PyResult<Expression> {
        Ok(Expression {
            expression: map_err(falcon::il::Expression::cmpeq(
                self.expression.clone(),
                other.expression,
            ))?,
        })
    }

    fn cmpneq(&self, other: Expression) -> PyResult<Expression> {
        Ok(Expression {
            expression: map_err(falcon::il::Expression::cmpneq(
                self.expression.clone(),
                other.expression,
            ))?,
        })
    }

    fn cmplts(&self, other: Expression) -> PyResult<Expression> {
        Ok(Expression {
            expression: map_err(falcon::il::Expression::cmplts(
                self.expression.clone(),
                other.expression,
            ))?,
        })
    }

    fn cmpltu(&self, other: Expression) -> PyResult<Expression> {
        Ok(Expression {
            expression: map_err(falcon::il::Expression::cmpltu(
                self.expression.clone(),
                other.expression,
            ))?,
        })
    }

    fn zext(&self, len: usize) -> PyResult<Expression> {
        Ok(Expression {
            expression: map_err(falcon::il::Expression::zext(len, self.expression.clone()))?,
        })
    }

    fn sext(&self, len: usize) -> PyResult<Expression> {
        Ok(Expression {
            expression: map_err(falcon::il::Expression::sext(len, self.expression.clone()))?,
        })
    }

    fn trun(&self, len: usize) -> PyResult<Expression> {
        Ok(Expression {
            expression: map_err(falcon::il::Expression::trun(len, self.expression.clone()))?,
        })
    }

    fn ite(&self, than: Expression, else_: Expression) -> PyResult<Expression> {
        Ok(Expression {
            expression: map_err(falcon::il::Expression::ite(
                self.expression.clone(),
                than.expression,
                else_.expression,
            ))?,
        })
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
