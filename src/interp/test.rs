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
