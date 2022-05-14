use nom::Finish;
use ruststep::{ast::*, parser::exchange, tables::*};
use serde::Deserialize;

espr_derive::inline_express!(
    r#"
    SCHEMA test_schema;
      ENTITY a;
        x: LIST [0:?] OF REAL;
      END_ENTITY;

      ENTITY b;
        a: LIST [0:?] OF a;
      END_ENTITY;

      ENTITY c;
        a: LIST [0:?] OF LIST [0:?] OF a;
      END_ENTITY;
      
      ENTITY d;
        a: LIST [0:?] OF LIST [0:?] OF LIST [0:?] OF a;
      END_ENTITY;
    END_SCHEMA;
    "#
);

use test_schema::*;

#[test]
fn deserialize_list_a() {
    let (residual, p): (_, SimpleEntityInstance) = exchange::simple_record("A((1.0, 2.0))").finish().unwrap();
    dbg!(&p);
    assert_eq!(residual, "");
    let a: AHolder = Deserialize::deserialize(&p).unwrap();
    dbg!(&a);
    assert_eq!(a, AHolder { x: vec![1.0, 2.0] });
}

#[test]
fn deserialize_list_b() {
    let (residual, p): (_, SimpleEntityInstance) = exchange::simple_record("B((A(((1.0)))))").finish().unwrap();
    dbg!(&p);
    assert_eq!(residual, "");
    let b: BHolder = Deserialize::deserialize(&p).unwrap();
    dbg!(&b);
    assert_eq!(
        b,
        BHolder {
            a: vec![PlaceHolder::Owned(AHolder { x: vec![1.0] })]
        }
    );
}

#[test]
fn deserialize_list_c() {
    let (residual, p): (_, SimpleEntityInstance) =
        exchange::simple_record("C( ( ( A(((1.0))) ), ( A(((2.0))) ) ) )")
            .finish()
            .unwrap();
    dbg!(&p);
    assert_eq!(residual, "");
    let c: CHolder = Deserialize::deserialize(&p).unwrap();
    dbg!(&c);
    assert_eq!(
        c,
        CHolder {
            a: vec![
                vec![PlaceHolder::Owned(AHolder { x: vec![1.0] })],
                vec![PlaceHolder::Owned(AHolder { x: vec![2.0] })]
            ]
        }
    );
}
