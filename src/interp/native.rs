use super::Interpreter;
use super::value::Value;
use itertools::Itertools;
use std::process;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use std::panic::panic_any;
use crate::interp::dict_key;
use regex::Regex;
use serde_json::Value as JSONValue;

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

pub fn exit(int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let code = match args.remove(0) {
        Value::Num(num) if num.trunc() == num => num as i32,
        _ => panic!("expected integer")
    };

    process::exit(code);
}

pub fn string(int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    Value::String(args.remove(0).to_string())
}

pub fn to_json(int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let json = JSONValue::from(args.remove(0));
    Value::String(json.to_string())
}

pub fn lower(int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let recv = if let Value::String(recv) = args.remove(0) {recv} else {unreachable!()};
    Value::String(recv.to_lowercase())
}

pub fn upper(int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let recv = if let Value::String(recv) = args.remove(0) {recv} else {unreachable!()};
    Value::String(recv.to_uppercase())
}

pub fn bool(int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let recv = if let Value::String(recv) = args.remove(0) {recv} else {unreachable!()};
    Value::Bool(recv.parse::<bool>().unwrap())
}

pub fn num(int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let recv = if let Value::String(recv) = args.remove(0) {recv} else {unreachable!()};
    Value::Num(recv.parse::<f64>().unwrap())
}

pub fn replace(int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let recv = if let Value::String(recv) = args.remove(0) {recv} else {unreachable!()};

    let (from, to) = match (args.remove(0), args.remove(0)) {
        (Value::String(from), Value::String(to)) => (from, to),
        _ => panic!("expected args to be two strings")
    };

    Value::String(recv.replace(&from, &to))
}

pub fn split(int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let recv = if let Value::String(recv) = args.remove(0) {recv} else {unreachable!()};

    let sep = match args.remove(0) {
        Value::String(sep) => sep,
        _ => panic!("expected arg to be string")
    };

    let vec: Vec<Value> = recv.split(&sep).map(|piece| Value::String(piece.to_string())).collect();

    Value::Vec(Rc::new(RefCell::new(vec)))
}

pub fn join(int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let recv = if let Value::String(recv) = args.remove(0) {recv} else {unreachable!()};

    let vec = match args.remove(0) {
        Value::Vec(vec) => vec,
        _ => panic!("expected arg to be vec")
    };

    let res = RefCell::borrow(&vec).iter().join(&recv);

    Value::String(res)
}

pub fn map(int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let recv = if let Value::Vec(recv) = args.remove(0) {recv} else {unreachable!()};
    let recv = RefCell::borrow(&recv);

    let f = args.remove(0);

    let vec: Vec<Value> = recv.iter().map(|val| int.call(f.clone(), vec![val.clone()])).collect();
    let vec = Rc::new(RefCell::new(vec));

    Value::Vec(vec)
}

pub fn filter(int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let recv = if let Value::Vec(recv) = args.remove(0) {recv} else {unreachable!()};
    let recv = RefCell::borrow(&recv);

    let f = args.remove(0);

    let vec: Vec<Value> = recv.iter().filter(|&val| int.call(f.clone(), vec![val.clone()]).is_truthy()).cloned().collect();
    let vec = Rc::new(RefCell::new(vec));

    Value::Vec(vec)
}

pub fn for_each(int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let recv = if let Value::Vec(recv) = args.remove(0) {recv} else {unreachable!()};
    let recv = RefCell::borrow(&recv);

    let f = args.remove(0);

    recv.iter().for_each(|val| {
        int.call(f.clone(), vec![val.clone()]);
    });

    Value::Nil
}

pub fn clone_vec(int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let recv = if let Value::Vec(recv) = args.remove(0) {recv} else {unreachable!()};
    let recv = RefCell::borrow(&recv);
    Value::Vec(Rc::new(RefCell::new(recv.clone())))
}

pub fn clone_dict(int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let recv = if let Value::Dict(recv) = args.remove(0) {recv} else {unreachable!()};
    let recv = RefCell::borrow(&recv);
    Value::Dict(Rc::new(RefCell::new(recv.clone())))
}

pub fn vec_2_dict(int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let recv = if let Value::Vec(recv) = args.remove(0) {recv} else {unreachable!()};
    let recv = RefCell::borrow(&recv);

    let mut map = HashMap::new();

    for tuple in recv.iter() {
        let mut tuple = if let Value::Vec(tuple) = tuple {
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

pub fn dict_2_vec(int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let recv = if let Value::Dict(recv) = args.remove(0) {recv} else {unreachable!()};
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

pub fn matches(int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let recv = if let Value::String(recv) = args.remove(0) {recv} else {unreachable!()};

    let pat = match args.remove(0) {
        Value::String(pat) => pat,
        _ => panic!("expected arg to be string")
    };

    let re = Regex::new(&pat).unwrap();

    Value::Bool(re.is_match(&recv))
}

pub fn find(int: &mut Interpreter, mut args: Vec<Value>) -> Value {
    let recv = if let Value::String(recv) = args.remove(0) {recv} else {unreachable!()};

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
