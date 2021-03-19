use std::collections::HashMap;

use super::Value;
use crate::interp::cmd::OsEnv;

pub struct Var {
    pub val: Value,
    pub is_exp: bool,
}

impl From<Value> for Var {
    fn from(val: Value) -> Self {
        Var {
            val,
            is_exp: false,
        }
    }
}

type Env = HashMap<String, Var>;

pub struct Stack(Vec<Env>);

impl Stack {
    pub fn new() -> Stack {
        Stack(vec![HashMap::new()])
    }

    pub fn get(&self, name: &str) -> &Value {
        for env in self.0.iter().rev() {
            match env.get(name) {
                Some(var) => return &var.val,
                None => continue,
            }
        }
        &Value::Nil
    }

    pub fn get_mut(&mut self, name: &str) -> &mut Value {
        for env in self.0.iter_mut().rev() {
            match env.get_mut(name) {
                Some(var) => return &mut var.val,
                None => continue,
            }
        }
        panic!("variable {} is undefined", name);
    }

    pub fn def<T: Into<Var>>(&mut self, name: String, var: T) {
        self.0.last_mut().unwrap().insert(name, var.into());
    }

    pub fn push(&mut self) {
        self.0.push(HashMap::new())
    }

    pub fn pop(&mut self) {
        self.0.pop();
    }

    pub fn os_env(&self) -> OsEnv {
        let mut out = Vec::new();

        for env in self.0.iter() {
            for var in env.iter() {
                if var.1.is_exp {
                    out.push((var.0.clone(), var.1.val.to_string()));
                }
            }
        }

        out
    }
}
