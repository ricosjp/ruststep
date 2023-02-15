use ruststep::ast::*;
use serde::Deserialize;

#[test]
fn deserialize_optional() {
    let p = Parameter::NotProvided;
    let x: Option<i32> = Deserialize::deserialize(&p).unwrap();
    assert_eq!(x, None);

    let p = Parameter::Integer(3);
    let x: Option<i32> = Deserialize::deserialize(&p).unwrap();
    assert_eq!(x, Some(3));
}
