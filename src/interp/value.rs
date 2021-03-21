use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use serde_json::{Value as JSONValue, Number as JSONNumber, Map as JSONMap};

use itertools::Itertools;

use crate::interp::func::Func;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Nil,
    Num(f64),
    String(String),
    Bool(bool),

    Vec(Rc<RefCell<Vec<Value>>>),
    Dict(Rc<RefCell<HashMap<String, Value>>>),

    Range(usize, usize),

    Func(Func),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Num(num) => write!(f, "{}", num),
            Value::String(string) => write!(f, "{}", string),
            Value::Bool(bool) => write!(f, "{}", bool),
            Value::Vec(vec) => {
                let vec = RefCell::borrow(vec);
                write!(f, "[{}]", vec.iter().map(|v| v.to_string_quoted()).join(", "))
            }
            Value::Dict(dict) => {
                let dict = RefCell::borrow(dict);
                write!(f, "{{{}}}", dict.iter().map(|(k, v)| format!("{}: {}", k, v.to_string_quoted())).join(", "))
            }
            Value::Func(func) => write!(f, "{:?}", func),
            Value::Range(l, r) => write!(f, "{}..{}", l, r),
        }
    }
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Bool(false) => false,
            _ => true,
        }
    }

    pub fn to_string_quoted(&self) -> String {
        if !matches!(self, Value::String(..)) {
            self.to_string()
        } else {
            format!("\'{}\'", self.to_string())
        }
    }
}

impl From<Value> for JSONValue {
    fn from(val: Value) -> Self {
        match val {
            Value::Nil => JSONValue::Null,
            Value::Num(num) => JSONValue::Number(JSONNumber::from_f64(num).unwrap()),
            Value::String(str) => JSONValue::String(str),
            Value::Bool(bool) => JSONValue::Bool(bool),
            Value::Vec(vec) => {
                let mut json_vec = Vec::new();

                for val in RefCell::borrow(&vec).iter() {
                    json_vec.push(val.clone().into());
                }

                JSONValue::Array(json_vec)
            }
            Value::Dict(map) => {
                let mut json_map = JSONMap::new();

                for (k, v) in RefCell::borrow(&map).iter() {
                    json_map.insert(k.clone(), v.clone().into());
                }

                JSONValue::Object(json_map)
            }
            _ => panic!("unserializable object")
        }
    }
}
