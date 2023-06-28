use super::Scope;
use crate::{
    error::Error,
    parser::ast::{self, Assign, Expression},
};

#[derive(Clone, Debug)]
pub enum Value {
    Primitive(Primitive),
    Error(String),
    Null,
}

impl Value {
    pub fn eval_assign(a: &Assign, scope: &mut Scope) -> Result<Self, Error> {
        let value = Value::eval_expr(&a.value, scope)?;
        scope.set(&a.name, &value);
        println!("{:?}", value);

        Ok(value)
    }

    pub fn eval_expr(e: &Expression, scope: &mut Scope) -> Result<Self, Error> {
        match e {
            Expression::Primitive(v) => Ok(Self::Primitive(Primitive::from(v))),
            Expression::Identifier(i) => match scope.get(i) {
                Some(v) => Ok(v.clone()),
                None => Ok(Self::Error(format!("undefined variable {}", i.value))),
            },
            Expression::Operator(_) => todo!(),
            Expression::Function(_) => todo!(),
            Expression::Call(_) => todo!(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Primitive {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
}

impl From<&ast::Primitive> for Primitive {
    fn from(value: &ast::Primitive) -> Self {
        match value {
            ast::Primitive::Integer(v) => Self::Integer(*v),
            ast::Primitive::Float(v) => Self::Float(*v),
            ast::Primitive::String(v) => Self::String(v.clone()),
            ast::Primitive::Boolean(v) => Self::Boolean(*v),
        }
    }
}
