use std::cell::RefCell;
use std::collections::HashMap;
use std::process;
use std::rc::Rc;

use itertools::Itertools;
use regex::Regex;
use serde_json::{from_str as json_from_str, Value as JSONValue};

use crate::interp::dict_key;

use super::Interpreter;
use super::value::Value;

pub fn print(int: &mut Interpreter, args: Vec<Value>) -> Value {
    let res = args.iter().map(|arg| arg.to_string()).join(" ");

    if let Some(str) = &mut int.collector {
        str.push_str(&res);
        str.push_str("\n");
    } else {
        println!("{}", res);
    }

    Value::Nil
}

pub fn exit(_: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let code = match args.remove(0) {
        Value::Num(num) if num.trunc() == num => num as i32,
        _ => panic!("expected integer")
    };

    process::exit(code);
}

pub fn string(_int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    Value::String(args.remove(0).to_string())
}

pub fn typ(_int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    Value::String(String::from(match args.remove(0) {
        Value::Nil => "nil",
        Value::Num(_) => "num",
        Value::String(_) => "string",
        Value::Bool(_) => "bool",
        Value::Vec(_) => "vec",
        Value::Dict(_) => "dict",
        Value::Range(_, _) => "range",
        Value::Func(_) => "func"
    }))
}

pub fn to_json(_int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let json = JSONValue::from(args.remove(0));
    Value::String(json.to_string())
}

pub fn from_json(_int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let recv = if let Value::String(recv) = args.remove(0) { recv } else { unreachable!() };
    let val: JSONValue = json_from_str(&recv).unwrap();
    Value::from(val)
}

pub fn lower(_int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let recv = if let Value::String(recv) = args.remove(0) { recv } else { unreachable!() };
    Value::String(recv.to_lowercase())
}

pub fn upper(_int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let recv = if let Value::String(recv) = args.remove(0) { recv } else { unreachable!() };
    Value::String(recv.to_uppercase())
}

pub fn bool(_int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let recv = if let Value::String(recv) = args.remove(0) { recv } else { unreachable!() };
    Value::Bool(recv.parse::<bool>().unwrap())
}

pub fn num(_int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let recv = if let Value::String(recv) = args.remove(0) { recv } else { unreachable!() };
    Value::Num(recv.parse::<f64>().unwrap())
}

pub fn replace(_int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let recv = if let Value::String(recv) = args.remove(0) { recv } else { unreachable!() };

    let (from, to) = match (args.remove(0), args.remove(0)) {
        (Value::String(from), Value::String(to)) => (from, to),
        _ => panic!("expected args to be two strings")
    };

    Value::String(recv.replace(&from, &to))
}

pub fn split(_int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let recv = if let Value::String(recv) = args.remove(0) { recv } else { unreachable!() };

    let sep = match args.remove(0) {
        Value::String(sep) => sep,
        _ => panic!("expected arg to be string")
    };

    let vec: Vec<Value> = recv.split(&sep).map(|piece| Value::String(piece.to_string())).collect();

    Value::Vec(Rc::new(RefCell::new(vec)))
}

pub fn join(_int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let recv = if let Value::String(recv) = args.remove(0) { recv } else { unreachable!() };

    let vec = match args.remove(0) {
        Value::Vec(vec) => vec,
        _ => panic!("expected arg to be vec")
    };

    let res = RefCell::borrow(&vec).iter().join(&recv);

    Value::String(res)
}

pub fn string_len(_int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let recv = if let Value::String(recv) = args.remove(0) { recv } else { unreachable!() };
    Value::Num(recv.len() as f64)
}

pub fn vec_len(_int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let recv = if let Value::Vec(recv) = args.remove(0) { recv } else { unreachable!() };
    let recv = RefCell::borrow(&recv);
    Value::Num(recv.len() as f64)
}

pub fn dict_len(_int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let recv = if let Value::Dict(recv) = args.remove(0) { recv } else { unreachable!() };
    let recv = RefCell::borrow(&recv);
    Value::Num(recv.len() as f64)
}

pub fn map(int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let recv = if let Value::Vec(recv) = args.remove(0) { recv } else { unreachable!() };
    let recv = RefCell::borrow(&recv);

    let f = args.remove(0);

    let vec: Vec<Value> = recv.iter().map(|val| int.call(f.clone(), vec![val.clone()])).collect();
    let vec = Rc::new(RefCell::new(vec));

    Value::Vec(vec)
}

pub fn filter(int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let recv = if let Value::Vec(recv) = args.remove(0) { recv } else { unreachable!() };
    let recv = RefCell::borrow(&recv);

    let f = args.remove(0);

    let vec: Vec<Value> = recv.iter().filter(|&val| int.call(f.clone(), vec![val.clone()]).is_truthy()).cloned().collect();
    let vec = Rc::new(RefCell::new(vec));

    Value::Vec(vec)
}

pub fn for_each(int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let recv = if let Value::Vec(recv) = args.remove(0) { recv } else { unreachable!() };
    let recv = RefCell::borrow(&recv);

    let f = args.remove(0);

    recv.iter().for_each(|val| {
        int.call(f.clone(), vec![val.clone()]);
    });

    Value::Nil
}

pub fn clone_vec(_int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let recv = if let Value::Vec(recv) = args.remove(0) { recv } else { unreachable!() };
    let recv = RefCell::borrow(&recv);
    Value::Vec(Rc::new(RefCell::new(recv.clone())))
}

pub fn clone_dict(_int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let recv = if let Value::Dict(recv) = args.remove(0) { recv } else { unreachable!() };
    let recv = RefCell::borrow(&recv);
    Value::Dict(Rc::new(RefCell::new(recv.clone())))
}

pub fn vec_2_dict(_int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let recv = if let Value::Vec(recv) = args.remove(0) { recv } else { unreachable!() };
    let recv = RefCell::borrow(&recv);

    let mut map = HashMap::new();

    for tuple in recv.iter() {
        let tuple = if let Value::Vec(tuple) = tuple {
            RefCell::borrow(&tuple)
        } else {
            panic!("expected all elements to be vec");
        };

        if tuple.len() != 2 {
            panic!("expected all elements to have len 2");
        }

        let key = dict_key(tuple[0].clone());
        map.insert(key, tuple[1].clone());
    }

    Value::Dict(Rc::new(RefCell::new(map)))
}

pub fn dict_2_vec(_int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let recv = if let Value::Dict(recv) = args.remove(0) { recv } else { unreachable!() };
    let recv = RefCell::borrow(&recv);

    let mut vec = Vec::new();

    for (k, v) in recv.iter() {
        vec.push(Value::Vec(Rc::new(RefCell::new(vec![
            Value::String(k.clone()),
            v.clone(),
        ]))));
    }

    Value::Vec(Rc::new(RefCell::new(vec)))
}

pub fn matches(_int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let recv = if let Value::String(recv) = args.remove(0) { recv } else { unreachable!() };

    let pat = match args.remove(0) {
        Value::String(pat) => pat,
        _ => panic!("expected arg to be string")
    };

    let re = Regex::new(&pat).unwrap();

    Value::Bool(re.is_match(&recv))
}

pub fn find(_int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let recv = if let Value::String(recv) = args.remove(0) { recv } else { unreachable!() };

    let pat = match args.remove(0) {
        Value::String(pat) => pat,
        _ => panic!("expected arg to be string")
    };

    let re = Regex::new(&pat).unwrap();

    let matches = re.captures_iter(&recv).map(|match_| {
        let groups = match_.iter().map(|group|
            Value::String(group.unwrap().as_str().to_string())
        ).collect::<Vec<Value>>();

        Value::Vec(Rc::new(RefCell::new(groups)))
    }).collect::<Vec<Value>>();

    Value::Vec(Rc::new(RefCell::new(matches)))
}

pub fn vec_remove(_int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let recv = if let Value::Vec(recv) = args.remove(0) { recv } else { unreachable!() };
    let mut recv = RefCell::borrow_mut(&recv);

    let index = match args.remove(0) {
        Value::Num(index) if index.trunc() == index => index as usize,
        _ => panic!("expected integer index")
    };

    recv.remove(index)
}

pub fn dict_remove(_int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let recv = if let Value::Dict(recv) = args.remove(0) { recv } else { unreachable!() };
    let mut recv = RefCell::borrow_mut(&recv);

    let index = dict_key(args.remove(0));

    recv.remove(&index).expect("key not found")
}
