// Test for deserializing Holder structs

use ruststep::{ast::*, parser::exchange, place_holder::PlaceHolder, tables::*};

use nom::Finish;
use serde::Deserialize;
use std::str::FromStr;

espr_derive::inline_express!(
    r#"
    SCHEMA test_schema;
      TYPE a = STRING;
      END_TYPE;

      TYPE b = ENUMERATION OF (
          are,
          sore,
          dore
        );
      END_TYPE;

      TYPE c = a;
      END_TYPE;

      TYPE d = b;
      END_TYPE;

      ENTITY e;
        a: a;
        b: b;
        c: c;
        d: d;
      END_ENTITY;
    END_SCHEMA;
    "#
);

use test_schema::*;

const EXAMPLE: &str = r#"
DATA;
  #1 = A('KORE');
  #2 = B(.ARE.);
  #3 = C(#1);
  #4 = D(.DORE.);
  #5 = E(#1, .SORE., #3, #4);
ENDSEC;
"#;

#[test]
fn deserialize_a_holder() {
    // from Record
    let (residual, p): (_, Record) = exchange::simple_record("A('KORE')").finish().unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let a: AHolder = Deserialize::deserialize(&p).unwrap();
    assert_eq!(a, AHolder("KORE".to_string()));

    // from Parameter::Typed
    let (residual, p): (_, Parameter) = exchange::parameter("A(('KORE'))").finish().unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let a: AHolder = Deserialize::deserialize(&p).unwrap();
    assert_eq!(a, AHolder("KORE".to_string()));

    // Test for PlaceHolder<AHolder>
    let (residual, p): (_, Record) = exchange::simple_record("A('KORE')").finish().unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let a: PlaceHolder<AHolder> = Deserialize::deserialize(&p).unwrap();
    assert_eq!(a, PlaceHolder::Owned(AHolder("KORE".to_string())));
}

#[test]
fn get_owned_a() {
    let table = Tables::from_str(EXAMPLE).unwrap();
    let a = EntityTable::<AHolder>::get_owned(&table, 1).unwrap();
    assert_eq!(a, A("KORE".to_string()));
}
