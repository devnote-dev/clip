use super::{value::Value, Scope};
use crate::{
    error::Error,
    parser::ast::{Operator, OperatorKind, Primitive},
};

pub fn eval_operator(op: Operator, scope: &mut Scope) -> Result<Value, Error> {
    if op.kind == OperatorKind::Inverse {
        if op.args.len() != 1 {
            return Err(Error::new(
                "expected exactly one argument for inverse operator",
            ));
        }

        return match Value::eval_expr(&op.args[0], scope)? {
            Value::Primitive(v) => match v {
                Primitive::Boolean(b) => Ok(Value::Primitive(Primitive::Boolean(!b))),
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
        OperatorKind::Add => eval_operator_add(values),
        OperatorKind::Subtract => eval_operator_subtract(values),
        OperatorKind::Multiply => eval_operator_multiply(values),
        OperatorKind::Divide => eval_operator_divide(values),
        OperatorKind::Equal => todo!(),
        OperatorKind::Inverse => unreachable!(),
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
                            "cannot add type integer with type {}",
                            arg
                        )))
                    }
                }
            }

            Ok(Value::Primitive(Primitive::Integer(res.iter().sum())))
        }
        Primitive::Float(val) => {
            let mut res = Vec::new();
            res.push(*val);

            for arg in values.iter().skip(1) {
                match arg {
                    Primitive::Float(v) => res.push(*v),
                    _ => {
                        return Err(Error::new(&format!(
                            "cannot add type float with type {}",
                            arg
                        )))
                    }
                }
            }

            Ok(Value::Primitive(Primitive::Float(res.iter().sum())))
        }
        Primitive::String(val) => {
            let mut res = val.clone();

            for arg in values.iter().skip(1) {
                match arg {
                    Primitive::String(v) => res.push_str(v),
                    _ => {
                        return Err(Error::new(&format!(
                            "cannot add type string with type {}",
                            arg
                        )))
                    }
                }
            }

            Ok(Value::Primitive(Primitive::String(res)))
        }
        val => Err(Error::new(&format!("cannot add type {}", val))),
    }
}

fn eval_operator_subtract(values: Vec<Primitive>) -> Result<Value, Error> {
    match &values[0] {
        Primitive::Integer(mut val) => {
            for arg in values.iter().skip(1) {
                match arg {
                    Primitive::Integer(v) => val -= v,
                    _ => {
                        return Err(Error::new(&format!(
                            "cannot subtract type integer with type {}",
                            arg
                        )))
                    }
                }
            }

            Ok(Value::Primitive(Primitive::Integer(val)))
        }
        Primitive::Float(mut val) => {
            for arg in values.iter().skip(1) {
                match arg {
                    Primitive::Float(v) => val -= v,
                    _ => {
                        return Err(Error::new(&format!(
                            "cannot subtract type float with type {}",
                            arg
                        )))
                    }
                }
            }

            Ok(Value::Primitive(Primitive::Float(val)))
        }
        val => Err(Error::new(&format!("cannot subtract type {}", val))),
    }
}

fn eval_operator_multiply(values: Vec<Primitive>) -> Result<Value, Error> {
    match &values[0] {
        Primitive::Integer(mut val) => {
            for arg in values.iter().skip(1) {
                match arg {
                    Primitive::Integer(v) => val *= v,
                    _ => {
                        return Err(Error::new(&format!(
                            "cannot multiply type integer with type {}",
                            arg
                        )))
                    }
                }
            }

            Ok(Value::Primitive(Primitive::Integer(val)))
        }
        Primitive::Float(mut val) => {
            for arg in values.iter().skip(1) {
                match arg {
                    Primitive::Float(v) => val *= v,
                    _ => {
                        return Err(Error::new(&format!(
                            "cannot multiply type float with type {}",
                            arg
                        )))
                    }
                }
            }

            Ok(Value::Primitive(Primitive::Float(val)))
        }
        val => Err(Error::new(&format!("cannot multiply type {}", val))),
    }
}

fn eval_operator_divide(values: Vec<Primitive>) -> Result<Value, Error> {
    match &values[0] {
        Primitive::Integer(mut val) => {
            for arg in values.iter().skip(1) {
                match arg {
                    Primitive::Integer(v) => val /= v,
                    _ => {
                        return Err(Error::new(&format!(
                            "cannot divide type integer with type {}",
                            arg
                        )))
                    }
                }
            }

            Ok(Value::Primitive(Primitive::Integer(val)))
        }
        Primitive::Float(mut val) => {
            for arg in values.iter().skip(1) {
                match arg {
                    Primitive::Float(v) => val /= v,
                    _ => {
                        return Err(Error::new(&format!(
                            "cannot divide type float with type {}",
                            arg
                        )))
                    }
                }
            }

            Ok(Value::Primitive(Primitive::Float(val)))
        }
        val => Err(Error::new(&format!("cannot divide type {}", val))),
    }
}
