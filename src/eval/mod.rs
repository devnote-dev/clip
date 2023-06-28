pub mod value;

use crate::{
    error::Error,
    parser::ast::{Identifier, Program, Statement},
};
use std::collections::HashMap;
use value::Value;

#[derive(Debug)]
pub struct Evaluator {
    statements: Vec<Statement>,
}

impl Evaluator {
    pub fn new(program: Program) -> Self {
        Self {
            statements: program.statements,
        }
    }

    pub fn eval(&self) -> Result<Value, Error> {
        let mut scope = Scope::default();
        let mut result = Value::Null;

        for stmt in &self.statements {
            match stmt {
                Statement::Assign(a) => result = Value::eval_assign(a, &mut scope)?,
                Statement::Expression(e) => result = Value::eval_expr(e, &mut scope)?,
            }
        }

        Ok(result)
    }
}

#[derive(Debug)]
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
