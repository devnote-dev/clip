use crate::{
    error::Error,
    parser::ast::{Identifier, Primitive, Program, Statement},
};
use std::collections::HashMap;
use value::Value;

pub mod value;

pub fn eval(program: Program, scope: &mut Scope) -> Result<Value, Error> {
    let mut result = Value::Primitive(Primitive::Null);

    for stmt in &program.statements {
        match stmt {
            Statement::Assign(a) => result = Value::eval_assign(a, scope)?,
            Statement::Expression(e) => result = Value::eval_expr(e, scope)?,
        }
    }

    Ok(result)
}

#[derive(Clone, Debug)]
pub struct Scope {
    store: HashMap<String, Value>,
    outer: Option<Box<Scope>>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            store: Default::default(),
            outer: None,
        }
    }

    pub fn get(&self, key: &Identifier) -> Option<&Value> {
        match self.store.get(&key.value) {
            Some(v) => Some(v),
            None => match &self.outer {
                Some(o) => o.get(key),
                None => None,
            },
        }
    }

    pub fn set(&mut self, key: &Identifier, value: &Value) {
        self.store.insert(key.value.clone(), value.clone());
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}
