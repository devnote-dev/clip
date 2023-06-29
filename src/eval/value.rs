use super::Scope;
use crate::{
    error::Error,
    parser::ast::{Assign, Expression, Function, Operator, OperatorKind, Primitive},
};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Primitive(Primitive),
    Function(Function),
    Null,
}

impl Value {
    pub fn eval_assign(a: &Assign, scope: &mut Scope) -> Result<Self, Error> {
        let value = Value::eval_expr(&a.value, scope)?;
        scope.set(&a.name, &value);

        Ok(value)
    }

    pub fn eval_expr(e: &Expression, scope: &mut Scope) -> Result<Self, Error> {
        match e {
            Expression::Primitive(v) => Ok(Self::Primitive(v.clone())),
            Expression::Identifier(i) => match scope.get(i) {
                Some(v) => Ok(v.clone()),
                None => Err(Error::new(&format!("undefined variable {}", i.value))),
            },
            Expression::Operator(v) => Value::eval_operator(v.clone(), scope),
            Expression::Function(v) => Ok(Self::Function(v.clone())),
            Expression::Call(_) => todo!(),
        }
    }

    fn eval_operator(op: Operator, scope: &mut Scope) -> Result<Self, Error> {
        if op.kind == OperatorKind::Inverse {
            if op.args.len() != 1 {
                return Err(Error::new(
                    "expected exactly one argument for inverse operator",
                ));
            }

            return match Value::eval_expr(&op.args[0], scope)? {
                Value::Primitive(v) => match v {
                    Primitive::Boolean(b) => Ok(Self::Primitive(Primitive::Boolean(!b))),
                    _ => Err(Error::new(&format!("cannot inverse type {}", v))),
                },
                t => Err(Error::new(&format!("cannot inverse type {}", t))),
            };
        }

        if op.args.len() < 2 {
            return Err(Error::new(&format!(
                "expected at least 2 arguments for {} operator",
                op.kind
            )));
        }

        let mut values = Vec::new();
        for arg in &op.args {
            match Value::eval_expr(arg, scope)? {
                Value::Primitive(v) => values.push(v),
                t => return Err(Error::new(&format!("cannot compare type {}", t))),
            }
        }

        match op.kind {
            OperatorKind::Add => Value::eval_operator_add(values),
            _ => todo!(),
        }
    }

    fn eval_operator_add(values: Vec<Primitive>) -> Result<Value, Error> {
        match &values[0] {
            Primitive::Integer(val) => {
                let mut res = Vec::new();
                res.push(*val);

                for arg in values.iter().skip(1) {
                    match arg {
                        Primitive::Integer(v) => res.push(*v),
                        _ => {
                            return Err(Error::new(&format!(
                                "cannot compare type integer with type {}",
                                arg
                            )))
                        }
                    }
                }

                Ok(Self::Primitive(Primitive::Integer(res.iter().sum())))
            }
            Primitive::Float(val) => {
                let mut res = Vec::new();
                res.push(*val);

                for arg in values.iter().skip(1) {
                    match arg {
                        Primitive::Float(v) => res.push(*v),
                        _ => {
                            return Err(Error::new(&format!(
                                "cannot compare type float with type {}",
                                arg
                            )))
                        }
                    }
                }

                Ok(Self::Primitive(Primitive::Float(res.iter().sum())))
            }
            Primitive::String(val) => {
                let mut res = val.clone();

                for arg in values.iter().skip(1) {
                    match arg {
                        Primitive::String(v) => res.push_str(v),
                        _ => {
                            return Err(Error::new(&format!(
                                "cannot compare type string with type {}",
                                arg
                            )))
                        }
                    }
                }

                Ok(Self::Primitive(Primitive::String(res)))
            }
            Primitive::Boolean(_) => Err(Error::new("cannot compare type boolean")),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Value::Primitive(p) => p.fmt(f),
            Value::Function(_) => write!(f, "function"),
            Value::Null => write!(f, "null"),
        }
    }
}
