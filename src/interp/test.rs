use std::fs;

use crate::lexer::new as new_lexer;
use crate::parser::Parser;

use super::*;
use super::value::Value;

fn output(source: &str, import_root: Option<PathBuf>) -> String {
    let lexer = new_lexer(source.to_owned());
    let mut parser = Parser::new(lexer);
    let prog = parser.parse();

    let mut interpreter = Interpreter::new();
    interpreter.do_collect();

    if let Some(import_root) = import_root {
        interpreter.set_import_root(import_root);
    }

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
    assert_eq!(output("print('ampere')", None), "ampere\n".to_string());
}

#[test]
fn global_variables() {
    assert_eq!(output("let name = 'ampere' print(name)", None), "ampere\n".to_string());
}

#[test]
fn scopes() {
    assert_eq!(output("let name = 'ampere' print(name) {let name = 'thomas the dank engine' print(name)} print(name)", None), "ampere\nthomas the dank engine\nampere\n".to_string());
}

#[test]
fn basic_output() {
    assert_eq!(output("echo xyz > /tmp/ampere ; cat /tmp/ampere", None), "xyz\n".to_string());
}

#[test]
fn assignment() {
    assert_eq!(output("let x = \"ampere\" x = \"xyz\" print(x)", None), "xyz\n".to_string());
}

#[test]
fn assignment_expr() {
    assert_eq!(output("let x print(x = \"xyz\")", None), "xyz\n".to_string());
}

#[test]
fn interp() {
    assert_eq!(output("let name = \"ampere\" print(\"x{name}x\")", None), "xamperex\n".to_string());
    assert_eq!(output("let name = \"ampere\" print(\"{name}\")", None), "ampere\n".to_string());
    assert_eq!(output("let name = \"ampere\" print(\"X{name}Y{name}Z\")", None), "XampereYampereZ\n".to_string());
}

#[test]
fn arithmetic() {
    assert_eq!(output("print(22+5)", None), "27\n".to_string());
    assert_eq!(output("print(22-5)", None), "17\n".to_string());
    assert_eq!(output("print(3*2)", None), "6\n".to_string());
    assert_eq!(output("print(8/2)", None), "4\n".to_string());
    assert_eq!(output("print(11%5)", None), "1\n".to_string());
    assert_eq!(output("print(2^4)", None), "16\n".to_string());
}

#[test]
fn arithmetic_assignment() {
    assert_eq!(output("let x = 17 x += 3 print(x)", None), "20\n".to_string());
    assert_eq!(output("let x = 17 x -= 2 print(x)", None), "15\n".to_string());
    assert_eq!(output("let x = 15 x *= 2 print(x)", None), "30\n".to_string());
    assert_eq!(output("let x = 6 x /= 2 print(x)", None), "3\n".to_string());
    assert_eq!(output("let x = 6 x %= 2 print(x)", None), "0\n".to_string());
    assert_eq!(output("let x = 6 x ^= 2 print(x)", None), "36\n".to_string());
}

#[test]
fn vecs_are_refs() {
    assert_eq!(output("let x = [1, 2, 3] let y = x y[0] = 99 print(x)", None), "[99, 2, 3]\n".to_string());
}

#[test]
fn dicts_are_refs() {
    assert_eq!(output("let x = {a: 1, b: 2, c: 3} let y = x y['a'] = 99 y.b=55 print(x.a, x.b, x.c)", None), "99 55 3\n".to_string());
}

#[test]
fn if_stmt() {
    assert_eq!(output("if true {print(1)}", None), "1\n".to_string());
}

#[test]
fn if_else_stmt() {
    assert_eq!(output("if false {print(1)} else {print(2)}", None), "2\n".to_string());
}

#[test]
fn if_else_if_else_stmt() {
    assert_eq!(output("if false {print(1)} else if true {print(2)} else {print(3)}", None), "2\n".to_string());
}

#[test]
fn exported_var() {
    assert_eq!(output("let AMPERE = 123\nenv | grep AMPERE", None), "".to_string());
    assert_eq!(output("exp let AMPERE = 123\nenv | grep AMPERE", None), "AMPERE=123\n".to_string());
}

#[test]
fn imported_var() {
    assert_eq!(output("print(USER + '\n' == $(whoami))", None), "true\n".to_string());
}

#[test]
fn continue_stmt() {
    assert_eq!(output("for i in 0..=2 {if i == 1 {continue} print(i)}", None), "0\n2\n".to_string());
}

#[test]
fn break_stmt() {
    assert_eq!(output("for i in 0..=2 {if i == 1 {break} print(i)}", None), "0\n".to_string());
}

#[test]
fn func() {
    assert_eq!(output("fn p() {print('ampere')} p()", None), "ampere\n".to_string());
}

#[test]
fn func_return() {
    assert_eq!(output("fn p(x) {return x^4} print(p(2))", None), "16\n".to_string());
}

#[test]
fn func_return_nil() {
    assert_eq!(output("fn p() {return} print(p())", None), "nil\n".to_string());
}

#[test]
fn func_no_return() {
    assert_eq!(output("fn p() {} print(p())", None), "nil\n".to_string());
}

#[test]
#[should_panic]
fn uncaught_break() {
    output("break", None);
}

#[test]
#[should_panic]
fn uncaught_return() {
    output("return", None);
}

#[test]
#[should_panic]
fn uncaught_break_func_call() {
    output("fn p() {break} p()", None);
}

#[test]
fn close_scope_escaping_func() {
    assert_eq!(output("fn p() {let i = 10 return} p() print(i)", None), "nil\n".to_string());
}

#[test]
fn close_scope_escaping_loop() {
    assert_eq!(output("while true { let i = 10 break } print(i)", None), "nil\n".to_string());
}

#[test]
fn func_cannot_access_outer_scope() {
    assert_eq!(output("fn f() {print(x)} let x = 10 f()", None), "nil\n".to_string());
}

#[test]
fn closure() {
    assert_eq!(output("let f { let i = 55 f = fn() { return i } } print(f())", None), "55\n".to_string());
}

#[test]
fn slices() {
    assert_eq!(output("let x = [1 2 3 4 5 6] let y = x[0..5] y[0] = 99 print(x) print(y)", None), "[1, 2, 3, 4, 5, 6]\n[99, 2, 3, 4, 5]\n".to_string());
}

#[test]
fn native_string() {
    assert_eq!(output("let x = [1 2 3].string() print(x + '!')", None), "[1, 2, 3]!\n".to_string());
}

#[test]
fn native_type() {
    assert_eq!(output("print([1 2 3].type())", None), "vec\n".to_string());
}

#[test]
fn native_to_json() {
    assert_eq!(output("print(['a', 'b'].toJson())", None), "[\"a\",\"b\"]\n".to_string());
}

#[test]
fn native_parse_json() {
    assert_eq!(output("print('[\"xyz\",\"b\"]'.parseJson()[0])", None), "xyz\n".to_string());
}

#[test]
fn native_upper() {
    assert_eq!(output("print('xyzXYZ123'.upper())", None), "XYZXYZ123\n".to_string());
}

#[test]
fn native_lower() {
    assert_eq!(output("print('xyzXYZ123'.lower())", None), "xyzxyz123\n".to_string());
}

#[test]
fn native_parse_bool() {
    assert_eq!(output("print(!'true'.parseBool())", None), "false\n".to_string());
}

#[test]
fn native_parse_num() {
    assert_eq!(output("print(2 * '4.1'.parseNum())", None), "8.2\n".to_string());
}

#[test]
fn native_replace() {
    assert_eq!(output("print('---xyz---'.replace('xyz', 'abc'))", None), "---abc---\n".to_string());
}

#[test]
fn native_split() {
    assert_eq!(output("print('u-v-www-x-y'.split('-')[2])", None), "www\n".to_string());
}

#[test]
fn native_join() {
    assert_eq!(output("print('-'.join(['a' 'b' 'c']))", None), "a-b-c\n".to_string());
    assert_eq!(output("print(''.join(['a' 'b' 'c']))", None), "abc\n".to_string());
}

#[test]
fn native_string_len() {
    assert_eq!(output("print('xyz'.len())", None), "3\n".to_string());
}

#[test]
fn native_vec_len() {
    assert_eq!(output("print([1 2 3].len())", None), "3\n".to_string());
}

#[test]
fn native_dict_len() {
    assert_eq!(output("print({a:1 b:2 c:3}.len())", None), "3\n".to_string());
}

#[test]
fn native_map() {
    assert_eq!(output("print([2 4 6].map(fn(i){return i^2}))", None), "[4, 16, 36]\n".to_string());
}

#[test]
fn native_filter() {
    assert_eq!(output("print([1 2 3 4 5 6].filter(fn(i){return i%2==0}))", None), "[2, 4, 6]\n".to_string());
}

#[test]
fn native_for_each() {
    assert_eq!(output("let i = 0 let x = [1 2 3 4 5 6] x.forEach(fn(_){i+=1}) print(i)", None), "6\n".to_string());
}

#[test]
fn native_clone_vec() {
    assert_eq!(output("let x = [1] let y = x.clone() y[0] = 99 print(x)", None), "[1]\n".to_string());
}

#[test]
fn native_clone_dict() {
    assert_eq!(output("let x = {a:1} let y = x.clone() y.a = 99 print(x)", None), "{a: 1}\n".to_string());
}

#[test]
fn native_vec_2_dict() {
    assert_eq!(output("print([['a', 1]].toDict())", None), "{a: 1}\n".to_string());
}

#[test]
fn native_dict_2_vec() {
    assert_eq!(output("print({a:1}.toVec())", None), "[['a', 1]]\n".to_string());
}

#[test]
fn native_matches() {
    assert_eq!(output("print('00AAA'.matches('\\d\\d\\w\\{3}'))", None), "true\n".to_string());
}

#[test]
fn native_find() {
    assert_eq!(output("print('01 23'.find('(\\d)(\\d)'))", None), "[['01', '0', '1'], ['23', '2', '3']]\n".to_string());
}

#[test]
fn native_vec_remove() {
    assert_eq!(output("let x = [1 2] print(x.remove(0)) print(x)", None), "1\n[2]\n".to_string());
}

#[test]
fn native_dict_remove() {
    assert_eq!(output("let x = {a: 1 b: 2} print(x.remove('a')) print(x)", None), "1\n{b: 2}\n".to_string());
}

#[test]
fn native_vec_sum() {
    assert_eq!(output("print([1] + [2])", None), "[1, 2]\n".to_string());
}

#[test]
fn native_dict_sum() {
    assert_eq!(output("print({} + {a:1})", None), "{a: 1}\n".to_string());
}

#[test]
fn native_strip() {
    assert_eq!(output("print('x\n')", None), "x\n\n".to_string());
    assert_eq!(output("print('x\n'.strip())", None), "x\n".to_string());
}

#[test]
fn native_string_contains() {
    assert_eq!(output("print('abcXXXdef'.contains('XXX'))", None), "true\n".to_string());
    assert_eq!(output("print('abcXYXdef'.contains('XXX'))", None), "false\n".to_string());
}

#[test]
fn native_vec_contains() {
    assert_eq!(output("print([1 2 3].contains(2))", None), "true\n".to_string());
}

#[test]
fn native_dict_contains() {
    assert_eq!(output("print({a:1 b:2 c:3}.contains('b'))", None), "true\n".to_string());
}

#[test]
fn test_golden() {
    let import_root = PathBuf::from("src/interp/golden");

    for path in fs::read_dir("src/interp/golden").unwrap() {
        let content = fs::read_to_string(path.unwrap().path()).unwrap();
        let (source, want) = {
            let split = content.split("\n#---\n").collect::<Vec<&str>>();
            (split[0].clone(), split[1].clone())
        };

        assert_eq!(output(source, Some(import_root.clone())), want);
    }
}

#[test]
fn native_bool() {
    assert_eq!(output("print(nil.bool())", None), "false\n".to_string());
    assert_eq!(output("print(false.bool())", None), "false\n".to_string());
    assert_eq!(output("print(123.bool())", None), "true\n".to_string());
}
