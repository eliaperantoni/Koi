use super::Value;
use std::collections::HashMap;

type Env = HashMap<String, Value>;

pub struct Stack(Vec<Env>);

impl Stack {
    pub fn new() -> Stack {
        Stack(vec![HashMap::new()])
    }

    pub fn globals(&mut self) -> &mut Env {
        self.0.first_mut().unwrap()
    }

    pub fn lookup(&self, name: &str) -> Value {
        for env in self.0.iter().rev() {
            match env.get(name) {
                Some(value) => return value.clone(),
                None => continue,
            }
        }
        Value::Nil
    }

    pub fn set(&mut self, name: String, value: Value) {
        self.0.last_mut().unwrap().insert(name, value);
    }

    pub fn push(&mut self) {
        self.0.push(HashMap::new())
    }

    pub fn pop(&mut self) {
        self.0.pop();
    }
}
