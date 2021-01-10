use super::*;

#[test]
fn vec_equality() {
    assert_eq!(Value::Vec(vec![Value::Num(1.0)]), Value::Vec(vec![Value::Num(1.0)]));
}

#[test]
fn dict_equality() {
    let mut a = HashMap::new();
    a.insert("foo".to_owned(), Value::Num(1.0));

    let mut b = HashMap::new();
    b.insert("foo".to_owned(), Value::Num(1.0));

    assert_eq!(Value::Dict(a), Value::Dict(b));
}

#[test]
fn to_string_vec() {
    let mut vec = Value::Vec(vec![]);
    assert_eq!(vec.to_string(), "[]");
    if let Value::Vec(vec) = &mut vec {
        vec.push(Value::Bool(true));
        vec.push(Value::Nil);
        vec.push(Value::String("foo".to_owned()));
    }
    assert_eq!(vec.to_string(), "[true, nil, 'foo']");
}

#[test]
fn to_string_dict() {
    let mut dict = Value::Dict(HashMap::new());
    assert_eq!(dict.to_string(), "{}");
    if let Value::Dict(dict) = &mut dict {
        dict.insert("foo".to_owned(), Value::Bool(true));
        dict.insert("bar".to_owned(), Value::String("baz".to_owned()));
    }
    assert!(["{foo: true, bar: 'baz'}", "{bar: 'baz', foo: true}"].contains(&&dict.to_string()[..]));
}
