extern crate test;
use test::Bencher;

use crate::lexer::new as new_lexer;
use crate::parser::Parser;

use super::*;
use super::value::Value;

fn output(source: &str) -> String {
    let lexer = new_lexer(source.to_owned());
    let mut parser = Parser::new(lexer);
    let prog = parser.parse();

    let mut interpreter = Interpreter::new();
    interpreter.do_collect();

    interpreter.run(prog);

    interpreter.collector.take().unwrap()
}

#[test]
fn vec_equality() {
    assert_eq!(Value::Vec(Rc::new(RefCell::new(vec![Value::Num(1.0)]))), Value::Vec(Rc::new(RefCell::new(vec![Value::Num(1.0)]))));
}

#[test]
fn dict_equality() {
    let mut a = HashMap::new();
    a.insert("foo".to_owned(), Value::Num(1.0));

    let mut b = HashMap::new();
    b.insert("foo".to_owned(), Value::Num(1.0));

    assert_eq!(Value::Dict(Rc::new(RefCell::new(a))), Value::Dict(Rc::new(RefCell::new(b))));
}

#[test]
fn to_string_vec() {
    let mut vec = Value::Vec(Rc::new(RefCell::new(Vec::new())));
    assert_eq!(vec.to_string(), "[]");

    if let Value::Vec(vec) = &mut vec {
        vec.borrow_mut().push(Value::Bool(true));
        vec.borrow_mut().push(Value::Nil);
        vec.borrow_mut().push(Value::String("foo".to_owned()));
    }
    assert_eq!(vec.to_string(), "[true, nil, 'foo']");
}

#[test]
fn to_string_dict() {
    let mut dict = Value::Dict(Rc::new(RefCell::new(HashMap::new())));
    assert_eq!(dict.to_string(), "{}");
    if let Value::Dict(dict) = &mut dict {
        dict.borrow_mut().insert("foo".to_owned(), Value::Bool(true));
        dict.borrow_mut().insert("bar".to_owned(), Value::String("baz".to_owned()));
    }
    assert!(["{foo: true, bar: 'baz'}", "{bar: 'baz', foo: true}"].contains(&&dict.to_string()[..]));
}

#[test]
fn print() {
    assert_eq!(output("print('ampere')"), "ampere\n".to_string());
}

#[test]
fn global_variables() {
    assert_eq!(output("let name = 'ampere' print(name)"), "ampere\n".to_string());
}

#[test]
fn scopes() {
    assert_eq!(output("let name = 'ampere' print(name) {let name = 'thomas the dank engine' print(name)} print(name)"), "ampere\nthomas the dank engine\nampere\n".to_string());
}

#[test]
fn basic_output() {
    assert_eq!(output("echo xyz > /tmp/ampere ; cat /tmp/ampere"), "xyz\n".to_string());
}

#[test]
fn assignment() {
    assert_eq!(output("let x = \"ampere\" x = \"xyz\" print(x)"), "xyz\n".to_string());
}

#[test]
fn assignment_expr() {
    assert_eq!(output("let x print(x = \"xyz\")"), "xyz\n".to_string());
}

#[test]
fn interp() {
    assert_eq!(output("let name = \"ampere\" print(\"x{name}x\")"), "xamperex\n".to_string());
    assert_eq!(output("let name = \"ampere\" print(\"{name}\")"), "ampere\n".to_string());
    assert_eq!(output("let name = \"ampere\" print(\"X{name}Y{name}Z\")"), "XampereYampereZ\n".to_string());
}

#[test]
fn arithmetic() {
    assert_eq!(output("print(22+5)"), "27\n".to_string());
    assert_eq!(output("print(22-5)"), "17\n".to_string());
    assert_eq!(output("print(3*2)"), "6\n".to_string());
    assert_eq!(output("print(8/2)"), "4\n".to_string());
    assert_eq!(output("print(11%5)"), "1\n".to_string());
    assert_eq!(output("print(2^4)"), "16\n".to_string());
}

#[test]
fn arithmetic_assignment() {
    assert_eq!(output("let x = 17 x += 3 print(x)"), "20\n".to_string());
    assert_eq!(output("let x = 17 x -= 2 print(x)"), "15\n".to_string());
    assert_eq!(output("let x = 15 x *= 2 print(x)"), "30\n".to_string());
    assert_eq!(output("let x = 6 x /= 2 print(x)"), "3\n".to_string());
    assert_eq!(output("let x = 6 x %= 2 print(x)"), "0\n".to_string());
    assert_eq!(output("let x = 6 x ^= 2 print(x)"), "36\n".to_string());
}

#[test]
fn vecs_are_refs() {
    assert_eq!(output("let x = [1, 2, 3] let y = x y[0] = 99 print(x)"), "[99, 2, 3]\n".to_string());
}

#[test]
fn dicts_are_refs() {
    assert_eq!(output("let x = {a: 1, b: 2, c: 3} let y = x y['a'] = 99 y.b=55 print(x.a, x.b, x.c)"), "99 55 3\n".to_string());
}

#[test]
fn if_stmt() {
    assert_eq!(output("if true {print(1)}"), "1\n".to_string());
}

#[test]
fn if_else_stmt() {
    assert_eq!(output("if false {print(1)} else {print(2)}"), "2\n".to_string());
}

#[test]
fn if_else_if_else_stmt() {
    assert_eq!(output("if false {print(1)} else if true {print(2)} else {print(3)}"), "2\n".to_string());
}

#[test]
fn exported_var() {
    assert_eq!(output("let AMPERE = 123\nenv | grep AMPERE"), "".to_string());
    assert_eq!(output("exp let AMPERE = 123\nenv | grep AMPERE"), "AMPERE=123\n".to_string());
}

#[test]
fn imported_var() {
    assert_eq!(output("print(USER + '\n' == $(whoami))"), "true\n".to_string());
}

#[test]
fn continue_stmt() {
    assert_eq!(output("for i in 0..=2 {if i == 1 {continue} print(i)}"), "0\n2\n".to_string());
}

#[test]
fn break_stmt() {
    assert_eq!(output("for i in 0..=2 {if i == 1 {break} print(i)}"), "0\n".to_string());
}

#[test]
fn func() {
    assert_eq!(output("fn p() {print('ampere')} p()"), "ampere\n".to_string());
}

#[test]
fn func_return() {
    assert_eq!(output("fn p(x) {return x^4} print(p(2))"), "16\n".to_string());
}

#[test]
fn func_return_nil() {
    assert_eq!(output("fn p() {return} print(p())"), "nil\n".to_string());
}

#[test]
fn func_no_return() {
    assert_eq!(output("fn p() {} print(p())"), "nil\n".to_string());
}

#[test]
#[should_panic]
fn uncaught_break() {
    output("break");
}

#[test]
#[should_panic]
fn uncaught_return() {
    output("return");
}

#[test]
#[should_panic]
fn uncaught_break_func_call() {
    output("fn p() {break} p()");
}

#[test]
fn close_scope_escaping_func() {
    assert_eq!(output("fn p() {let i = 10 return} p() print(i)"), "nil\n".to_string());
}

#[test]
fn close_scope_escaping_loop() {
    assert_eq!(output("while true { let i = 10 break } print(i)"), "nil\n".to_string());
}

#[test]
fn func_cannot_access_outer_scope() {
    assert_eq!(output("fn f() {print(x)} let x = 10 f()"), "nil\n".to_string());
}

#[test]
fn closure() {
    assert_eq!(output("let f { let i = 55 f = fn() { return i } } print(f())"), "55\n".to_string());
}
