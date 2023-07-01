use super::{ops, Scope};
use crate::{
    error::Error,
    parser::ast::{And, Assign, Call, Expression, Function, If, Or, Primitive, Statement},
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

    pub fn eval_if_condition(i: &If, scope: &mut Scope) -> Result<Self, Error> {
        let condition = match Value::eval_expr(&i.condition, scope)? {
            Value::Primitive(p) => match p {
                Primitive::Boolean(v) => v,
                Primitive::Null => false,
                _ => true,
            },
            Value::Function(_) => {
                return Err(Error::new("cannot use type function as a condition"))
            }
        };

        if condition {
            for cons in &i.consequence {
                match cons.as_ref() {
                    Statement::Assign(v) => Value::eval_assign(v, scope)?,
                    Statement::If(v) => Value::eval_if_condition(v, scope)?,
                    Statement::Expression(v) => Value::eval_expr(v, scope)?,
                };
            }
        } else if let Some(alternative) = &i.alternative {
            for alt in alternative {
                match alt.as_ref() {
                    Statement::Assign(v) => Value::eval_assign(v, scope)?,
                    Statement::If(v) => Value::eval_if_condition(v, scope)?,
                    Statement::Expression(v) => Value::eval_expr(v, scope)?,
                };
            }
        }

        Ok(Self::Primitive(Primitive::Null))
    }

    pub fn eval_expr(e: &Expression, scope: &mut Scope) -> Result<Self, Error> {
        match e {
            Expression::Primitive(v) => Ok(Self::Primitive(v.clone())),
            Expression::Identifier(i) => match scope.get(i) {
                Some(v) => Ok(v.clone()),
                None => Err(Error::new(&format!("undefined variable {}", i.value))),
            },
            Expression::Operator(v) => ops::eval_operator(v.clone(), scope),
            Expression::Function(v) => Ok(Self::Function(v.clone())),
            Expression::Call(v) => Value::eval_call(v.clone(), scope),
            Expression::And(v) => Value::eval_logic_and(v.clone(), scope),
            Expression::Or(v) => Value::eval_logic_or(v.clone(), scope),
        }
    }

    fn eval_call(call: Call, scope: &mut Scope) -> Result<Self, Error> {
        let Some(val) = scope.get(&call.name) else {
            return Err(Error::new(&format!("undefined function variable {}", call.name.value)));
        };

        match val {
            Value::Function(fun) => {
                if call.args.len() != fun.params.len() {
                    if call.args.len() == 1 && fun.params.is_empty() {
                        match &call.args[0] {
                            Expression::Primitive(Primitive::Null) => (),
                            _ => {
                                return Err(Error::new(&format!(
                                    "function {} can only be called with ()",
                                    call.name.value
                                )))
                            }
                        }
                    } else {
                        return Err(Error::new(&format!(
                            "expected {} arguments to function {}",
                            fun.params.len(),
                            call.name.value
                        )));
                    }
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
                        Statement::If(i) => result = Self::eval_if_condition(i, &mut child)?,
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

    fn eval_logic_and(and: And, scope: &mut Scope) -> Result<Self, Error> {
        let mut values = Vec::new();

        for expr in &and.0 {
            values.push(Value::eval_expr(expr, scope)?);
        }

        for val in values {
            match val {
                Value::Primitive(p) => match p {
                    Primitive::Boolean(v) if !v => {
                        return Ok(Value::Primitive(Primitive::Boolean(false)));
                    }
                    Primitive::Null => return Ok(Value::Primitive(Primitive::Boolean(false))),
                    _ => (),
                },
                Value::Function(_) => (),
            }
        }

        Ok(Value::Primitive(Primitive::Boolean(true)))
    }

    fn eval_logic_or(or: Or, scope: &mut Scope) -> Result<Self, Error> {
        let mut values = Vec::new();

        for expr in &or.0 {
            values.push(Value::eval_expr(expr, scope)?);
        }

        for val in values {
            match val {
                Value::Primitive(p) => match p {
                    Primitive::Boolean(v) if !v => (),
                    Primitive::Null => (),
                    _ => return Ok(Value::Primitive(Primitive::Boolean(true))),
                },
                Value::Function(_) => return Ok(Value::Primitive(Primitive::Boolean(true))),
            }
        }

        Ok(Value::Primitive(Primitive::Boolean(false)))
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
