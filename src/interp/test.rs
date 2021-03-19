use super::*;
use crate::parser::Parser;
use crate::lexer::new as new_lexer;

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
fn pre_post_inc_dec() {
    assert_eq!(output("let x = 5 print(x++) print(x)"), "5\n6\n".to_string());
    assert_eq!(output("let x = 5 print(x--) print(x)"), "5\n4\n".to_string());
    assert_eq!(output("let x = 5 print(++x) print(x)"), "6\n6\n".to_string());
    assert_eq!(output("let x = 5 print(--x) print(x)"), "4\n4\n".to_string());
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
