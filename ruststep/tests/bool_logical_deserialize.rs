use ruststep::{ast::*, primitive::*};
use serde::Deserialize;

fn sub_deserialize<T>(param: &str, ans: T)
where
    T: std::fmt::Debug + PartialEq + Deserialize<'static>,
{
    let p = Parameter::Enumeration(param.to_string());
    let x: T = Deserialize::deserialize(&p).unwrap();
    assert_eq!(x, ans);
}

#[test]
fn bool_deserialize() {
    sub_deserialize("T", true);
    sub_deserialize("TRUE", true);
    sub_deserialize("F", false);
    sub_deserialize("FALSE", false);

    let p = Parameter::Enumeration("UNKNOWN".to_string());
    assert!(bool::deserialize(&p).is_err());
}

#[test]
fn deserialize_logical() {
    sub_deserialize("T", Logical::True);
    sub_deserialize("TRUE", Logical::True);
    sub_deserialize("F", Logical::False);
    sub_deserialize("FALSE", Logical::False);
    sub_deserialize("U", Logical::Unknown);
    sub_deserialize("Unknown", Logical::Unknown);

    let p = Parameter::Enumeration("Q".to_string());
    assert!(Logical::deserialize(&p).is_err());
}
