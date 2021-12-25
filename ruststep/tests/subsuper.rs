use nom::Finish;
use ruststep::{ast::*, parser::exchange, tables::*};
use serde::Deserialize;
use std::str::FromStr;

espr_derive::inline_express!(
    r#"
    SCHEMA test_schema;
      ENTITY base SUPERTYPE OF (ONEOF (sub));
        x: REAL;
      END_ENTITY;

      ENTITY sub
        SUPERTYPE OF (subsub)
        SUBTYPE OF (base);
        y: REAL;
      END_ENTITY;

      ENTITY subsub SUBTYPE OF (sub);
        z: REAL;
      END_ENTITY;
    END_SCHEMA;
    "#
);

use test_schema::*;

#[test]
fn deserialize_base() {
    let (residual, p): (_, Record) = exchange::simple_record("BASE(1.0)").finish().unwrap();
    dbg!(&p);
    assert_eq!(residual, "");
    let a: BaseHolder = Deserialize::deserialize(&p).unwrap();
    dbg!(&a);
    assert_eq!(a, BaseHolder { x: 1.0 });
}

#[test]
fn deserialize_sub() {
    test(
        "SUB(BASE((1.0)), 2.0)",
        SubHolder {
            base: BaseHolder { x: 1.0 }.into(),
            y: 2.0,
        },
    );
    test(
        "SUB(#3, 2.0)",
        SubHolder {
            base: RValue::Entity(3).into(),
            y: 2.0,
        },
    );
    fn test(input: &str, answer: SubHolder) {
        let (residual, p): (_, Record) = exchange::simple_record(input).finish().unwrap();
        dbg!(&p);
        assert_eq!(residual, "");
        let a: SubHolder = Deserialize::deserialize(&p).unwrap();
        dbg!(&a);
        assert_eq!(a, answer);
    }
}

#[test]
fn deserialize_subsub() {
    test(
        "SUBSUB(SUB((BASE((1.0)), 2.0)), 3.0)",
        SubsubHolder {
            sub: SubHolder {
                base: BaseHolder { x: 1.0 }.into(),
                y: 2.0,
            }
            .into(),
            z: 3.0,
        },
    );
    fn test(input: &str, answer: SubsubHolder) {
        let (residual, p): (_, Record) = exchange::simple_record(input).finish().unwrap();
        dbg!(&p);
        assert_eq!(residual, "");
        let a: SubsubHolder = Deserialize::deserialize(&p).unwrap();
        dbg!(&a);
        assert_eq!(a, answer);
    }
}

const EXAMPLE: &str = r#"
DATA;
  #1 = BASE(1.0);
  #2 = SUB(BASE((1.0)), 2.0);
  #3 = SUBSUB(#2, 4.0);
ENDSEC;
"#;

#[test]
fn get_owned_base() {
    let table = Tables::from_str(EXAMPLE).unwrap();
    let owned = EntityTable::<BaseHolder>::get_owned(&table, 1).unwrap();
    assert_eq!(owned, Base { x: 1.0 });
}

#[test]
fn get_owned_sub() {
    let table = Tables::from_str(EXAMPLE).unwrap();
    let owned = EntityTable::<SubHolder>::get_owned(&table, 2).unwrap();
    assert_eq!(
        owned,
        Sub {
            base: Base { x: 1.0 },
            y: 2.0
        }
    );
}

#[test]
fn get_owned_subsub() {
    let table = Tables::from_str(EXAMPLE).unwrap();
    let owned = EntityTable::<SubsubHolder>::get_owned(&table, 3).unwrap();
    assert_eq!(
        owned,
        Subsub {
            sub: Sub {
                base: Base { x: 1.0 },
                y: 2.0,
            },
            z: 4.0,
        }
    );
}

#[test]
fn get_owned_any() {
    let table = Tables::from_str(EXAMPLE).unwrap();

    // BaseAny
    let any = EntityTable::<BaseAnyHolder>::get_owned(&table, 1).unwrap();
    assert_eq!(any, BaseAny::Base(Box::new(Base { x: 1.0 })));
    let any = EntityTable::<BaseAnyHolder>::get_owned(&table, 2).unwrap();
    assert_eq!(
        any,
        BaseAny::Sub(Box::new(
            Sub {
                base: Base { x: 1.0 },
                y: 2.0
            }
            .into()
        ))
    );

    // SubAny
    let any = EntityTable::<SubAnyHolder>::get_owned(&table, 2).unwrap();
    assert_eq!(
        any,
        SubAny::Sub(Box::new(
            Sub {
                base: Base { x: 1.0 },
                y: 2.0
            }
            .into()
        ))
    );
    let any = EntityTable::<SubAnyHolder>::get_owned(&table, 3).unwrap();
    assert_eq!(
        any,
        SubAny::Subsub(Box::new(
            Subsub {
                sub: Sub {
                    base: Base { x: 1.0 },
                    y: 2.0,
                },
                z: 4.0
            }
            .into()
        ))
    );
}
