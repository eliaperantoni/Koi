use crate::interp::Value;
use std::collections::HashMap;

#[derive(Clone)]
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

pub type Env = HashMap<String, Var>;
