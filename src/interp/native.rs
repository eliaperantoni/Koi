use super::Interpreter;
use super::value::Value;
use itertools::Itertools;
use std::process;

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
