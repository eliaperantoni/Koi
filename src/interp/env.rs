use std::collections::HashMap;
use super::Value;

pub type Stack = Vec<Env>;

pub type Env = HashMap<String, Value>;

impl Env {
    pub fn new() -> Env {
        HashMap::new()
    }

    pub fn get() -> Value {

    }
}
