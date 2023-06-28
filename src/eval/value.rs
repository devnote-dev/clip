use super::Scope;
use crate::{
    error::Error,
    parser::ast::{Assign, Expression, Function, Operator, OperatorKind, Primitive},
};

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
                    _ => Err(Error::new(&format!("cannot inverse type {:?}", v))),
                },
                t => Err(Error::new(&format!("cannot inverse type {:?}", t))),
            };
        }

        if op.args.len() < 2 {
            return Err(Error::new(&format!(
                "expected at least 2 arguments for {:?} operator",
                op.kind
            )));
        }

        let mut values = Vec::new();
        for arg in &op.args {
            match Value::eval_expr(arg, scope)? {
                Value::Primitive(v) => values.push(v),
                t => return Err(Error::new(&format!("cannot compare type {:?}", t))),
            }
        }

        let head = &values[0];
        if !values.iter().all(|v| head == v) {
            return Err(Error::new(&format!(
                "expected all arguments to be of type {:?}",
                head
            )));
        }

        match op.kind {
            OperatorKind::Add => Value::eval_operator_add(values),
            _ => todo!(),
        }
    }

    fn eval_operator_add(values: Vec<Primitive>) -> Result<Value, Error> {
        let result = &values[0];

        // for val in values.iter().skip(1) {
        //     match val {
        //         Primitive::Integer(v) => result = Primitive::Integer(result + v),
        //         Primitive::Float(v) => result = Primitive::Float(result + v),
        //         Primitive::String(v) => result = Primitive::String(result + v),
        //         _ => return Err(Error::new(&format!("cannot add type {:?}", val))),
        //     }
        // }

        Ok(Self::Primitive(result.clone()))
    }
}
