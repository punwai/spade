use std::{collections::HashMap, hash::Hash};
use crate::evaluate::Value;

pub struct Environment {
    stack: Vec<HashMap<String, Value>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            stack: vec![HashMap::new()],
        }
    }

    pub fn new_child(env: &Environment) -> Self {
        let mut new_stack = env.stack.clone();
        new_stack.push(HashMap::new());
        Environment { 
            stack: new_stack,
        }
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn define(&mut self, name: String, value: Value) {
        if let Some(current_scope) = self.stack.last_mut() {
            current_scope.insert(name, value);
        }
    }

    pub fn get(&self, name: &str) -> Result<Value, String> {
        for scope in self.stack.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Ok(value.clone());
            }
        }
        Err(format!("Undefined variable '{}'.", name))
    }

    pub fn assign(&mut self, name: String, value: Value) -> Result<(), String> {
        for scope in self.stack.iter_mut().rev() {
            if scope.contains_key(&name) {
                scope.insert(name, value);
                return Ok(());
            }
        }
        Err(format!("Undefined variable '{}'.", name))
    }
}