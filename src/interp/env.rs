use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::DerefMut;
use std::rc::Rc;

use crate::interp::value::Value;
use crate::interp::cmd::OsEnv;

pub struct Var {
    val: Value,
    is_exp: bool,
}

impl Var {
    pub fn new(val: Value, is_exp: bool) -> Var {
        Var {
            val,
            is_exp,
        }
    }
}

impl From<Value> for Var {
    fn from(val: Value) -> Self {
        Var {
            val,
            is_exp: false,
        }
    }
}

pub struct Env {
    map: HashMap<String, Var>,
    parent: Option<Rc<RefCell<Env>>>,
}

impl Env {
    pub fn new() -> Env {
        Env {
            map: HashMap::new(),
            parent: None,
        }
    }

    pub fn new_from(parent: &Rc<RefCell<Env>>) -> Env {
        Env {
            map: HashMap::new(),
            parent: Some(Rc::clone(parent)),
        }
    }

    pub fn parent_ref(&self) -> Rc<RefCell<Env>> {
        Rc::clone(self.parent.as_ref().unwrap())
    }

    pub fn get(&self, name: &str) -> Value {
        if let Some(val) = self.map.get(name) {
            val.val.clone()
        } else {
            if let Some(parent) = &self.parent {
                RefCell::borrow(parent).get(name)
            } else {
                Value::Nil
            }
        }
    }

    pub fn put(&mut self, name: &str, new_val: Value) {
        if let Some(val) = self.map.get_mut(name) {
            val.val = new_val
        } else {
            if let Some(parent) = &mut self.parent {
                RefCell::borrow_mut(parent).put(name, new_val)
            } else {
                panic!("undefined variable");
            }
        }
    }

    pub fn def<T: Into<Var>>(&mut self, name: String, var: T) {
        self.map.insert(name, var.into());
    }

    pub fn os_env(&self) -> OsEnv {
        let mut os_env = if let Some(parent) = &self.parent {
            RefCell::borrow(parent).os_env()
        } else {
            OsEnv::new()
        };

        for (k, v) in self.map.iter() {
            if v.is_exp {
                os_env.push((k.clone(), v.val.to_string()));
            }
        }

        os_env
    }
}
