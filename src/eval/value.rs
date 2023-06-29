use super::Scope;
use crate::{
    error::Error,
    parser::ast::{
        Assign, Call, Expression, Function, Operator, OperatorKind, Primitive, Statement,
    },
};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Primitive(Primitive),
    Function(Function),
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
            Expression::Call(v) => Value::eval_call(v.clone(), scope),
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
            val => Err(Error::new(&format!("cannot compare type {}", val))),
        }
    }

    fn eval_call(call: Call, scope: &mut Scope) -> Result<Self, Error> {
        let Some(val) = scope.get(&call.name) else {
            return Err(Error::new(&format!("undefined function variable {}", call.name.value)));
        };

        match val {
            Value::Function(fun) => {
                if call.args.len() != fun.params.len() {
                    return Err(Error::new(&format!(
                        "expected {} arguments to function {}",
                        fun.params.len(),
                        call.name.value
                    )));
                }

                let mut child = Scope {
                    store: Default::default(),
                    outer: Some(Box::new(scope.clone())),
                };

                for (param, expr) in fun.params.iter().zip(call.args.iter()) {
                    let v = &Value::eval_expr(expr, &mut child)?;
                    child.set(param, v);
                }

                let mut result = Self::Primitive(Primitive::Null);

                for stmt in &fun.body {
                    match stmt {
                        Statement::Assign(a) => result = Self::eval_assign(a, &mut child)?,
                        Statement::Expression(e) => result = Self::eval_expr(e, &mut child)?,
                    }
                }

                Ok(result)
            }
            Value::Primitive(p) => {
                Err(Error::new(&format!("cannot call type {} as a function", p)))
            }
        }
    }

    pub fn value(&self) -> String {
        match self {
            Value::Primitive(p) => match p {
                Primitive::Integer(v) => v.to_string(),
                Primitive::Float(v) => v.to_string(),
                Primitive::String(v) => v.to_string(),
                Primitive::Boolean(v) => v.to_string(),
                Primitive::Null => "null".to_string(),
            },
            Value::Function(_) => "function".to_string(),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Value::Primitive(p) => p.fmt(f),
            Value::Function(_) => write!(f, "function"),
        }
    }
}
