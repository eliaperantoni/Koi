use super::cross_product;
use super::super::Value;

#[test]
fn test_cross_product() {
    assert_eq!(cross_product(vec![
        Value::String(String::from("x")),
        Value::String(String::from("y")),
        Value::String(String::from("z")),
    ]), vec![
        String::from("xyz"),
    ]);

    assert_eq!(cross_product(vec![
        Value::String(String::from("x")),
        Value::Vec(vec![
            Value::Num(1.0),
            Value::Num(2.0),
            Value::Num(3.0),
        ]),
        Value::String(String::from("y")),
    ]), vec![
        String::from("x1y"),
        String::from("x2y"),
        String::from("x3y"),
    ]);

    assert_eq!(cross_product(vec![
        Value::String(String::from("x")),
        Value::Vec(vec![
            Value::Num(1.0),
            Value::Num(2.0),
            Value::Num(3.0),
        ]),
        Value::String(String::from("y")),
        Value::Vec(vec![
            Value::Num(1.0),
            Value::Num(2.0),
            Value::Num(3.0),
        ]),
        Value::String(String::from("z")),
    ]), vec![
        String::from("x1y1z"),
        String::from("x1y2z"),
        String::from("x1y3z"),
        String::from("x2y1z"),
        String::from("x2y2z"),
        String::from("x2y3z"),
        String::from("x3y1z"),
        String::from("x3y2z"),
        String::from("x3y3z"),
    ]);
}
