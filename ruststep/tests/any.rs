use nom::Finish;
use ruststep::{ast::*, parser::exchange, place_holder::*, tables::*};
use serde::Deserialize;
use std::str::FromStr;

espr_derive::inline_express!(
    r#"
    SCHEMA test_schema;
      ENTITY base SUPERTYPE OF (ONEOF (sub1, sub2));
        x: REAL;
      END_ENTITY;

      ENTITY sub1 SUBTYPE OF (base);
        y1: REAL;
      END_ENTITY;

      ENTITY sub2 SUBTYPE OF (base);
        y2: REAL;
      END_ENTITY;
    END_SCHEMA;
    "#
);

use test_schema::*;

const EXAMPLE: &str = r#"
DATA;
  #1 = BASE(1.0);
  #2 = SUB_1(BASE((1.0)), 2.0);
  #3 = SUB_2(#1, 4.0);
ENDSEC;
"#;

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
fn deserialize_sub1() {
    test(
        "SUB_1(BASE((1.0)), 2.0)",
        Sub1Holder {
            base: BaseHolder { x: 1.0 }.into(),
            y1: 2.0,
        },
    );
    test(
        "SUB_1(#3, 2.0)",
        Sub1Holder {
            base: RValue::Entity(3).into(),
            y1: 2.0,
        },
    );

    fn test(input: &str, answer: Sub1Holder) {
        let (residual, p): (_, Record) = exchange::simple_record(input).finish().unwrap();
        dbg!(&p);
        assert_eq!(residual, "");

        let a: Sub1Holder = Deserialize::deserialize(&p).unwrap();
        dbg!(&a);
        assert_eq!(a, answer);
    }
}

#[test]
fn deserialize_base_any() {
    test(
        "SUB_1(BASE((1.0)), 2.0)",
        BaseAnyHolder::Sub1(Box::new(Sub1Holder {
            base: BaseHolder { x: 1.0 }.into(),
            y1: 2.0,
        })),
    );
    test(
        "SUB_1(#3, 2.0)",
        BaseAnyHolder::Sub1(Box::new(Sub1Holder {
            base: RValue::Entity(3).into(),
            y1: 2.0,
        })),
    );

    fn test(input: &str, answer: BaseAnyHolder) {
        let (residual, p): (_, Record) = exchange::simple_record(input).finish().unwrap();
        dbg!(&p);
        assert_eq!(residual, "");

        let a: BaseAnyHolder = Deserialize::deserialize(&p).unwrap();
        dbg!(&a);
        assert_eq!(a, answer);
    }
}

#[test]
fn deserialize_base_any_placeholder() {
    test(
        "SUB_1(BASE((1.0)), 2.0)",
        PlaceHolder::Owned(BaseAnyHolder::Sub1(Box::new(Sub1Holder {
            base: BaseHolder { x: 1.0 }.into(),
            y1: 2.0,
        }))),
    );
    test(
        "SUB_1(#3, 2.0)",
        PlaceHolder::Owned(BaseAnyHolder::Sub1(Box::new(Sub1Holder {
            base: RValue::Entity(3).into(),
            y1: 2.0,
        }))),
    );

    fn test(input: &str, answer: PlaceHolder<BaseAnyHolder>) {
        let (residual, p): (_, Record) = exchange::simple_record(input).finish().unwrap();
        dbg!(&p);
        assert_eq!(residual, "");

        let a: PlaceHolder<BaseAnyHolder> = Deserialize::deserialize(&p).unwrap();
        dbg!(&a);
        assert_eq!(a, answer);
    }
}

#[test]
fn into_base_any() {
    test(
        "SUB_1(BASE((1.0)), 2.0)",
        BaseAny::Sub1(Box::new(Sub1 {
            base: Base { x: 1.0 },
            y1: 2.0,
        })),
    );
    test(
        "SUB_1(#1, 2.0)",
        BaseAny::Sub1(Box::new(Sub1 {
            base: Base { x: 1.0 },
            y1: 2.0,
        })),
    );

    fn test(input: &str, answer: BaseAny) {
        let table = Tables::from_str(EXAMPLE).unwrap();

        let (residual, p): (_, Record) = exchange::simple_record(input).finish().unwrap();
        dbg!(&p);
        assert_eq!(residual, "");

        let holder = PlaceHolder::<BaseAnyHolder>::deserialize(&p).unwrap();
        dbg!(&holder);

        let owned = holder.into_owned(&table).unwrap();
        dbg!(&owned);
        assert_eq!(owned, answer);
    }
}

#[test]
fn lookup_base_any() {
    test(
        Parameter::RValue(RValue::Entity(1)),
        BaseAny::Base(Box::new(Base { x: 1.0 })),
    );
    test(
        Parameter::RValue(RValue::Entity(2)),
        BaseAny::Sub1(Box::new(Sub1 {
            base: Base { x: 1.0 },
            y1: 2.0,
        })),
    );
    test(
        Parameter::RValue(RValue::Entity(3)),
        BaseAny::Sub2(Box::new(Sub2 {
            base: Base { x: 1.0 },
            y2: 4.0,
        })),
    );

    fn test(p: Parameter, answer: BaseAny) {
        let table = Tables::from_str(EXAMPLE).unwrap();

        let holder = PlaceHolder::<BaseAnyHolder>::deserialize(&p).unwrap();
        dbg!(&holder);

        let owned = holder.into_owned(&table).unwrap();
        dbg!(&owned);
        assert_eq!(owned, answer);
    }
}

#[test]
fn get_owned_base() {
    let table = Tables::from_str(EXAMPLE).unwrap();
    let base = EntityTable::<BaseHolder>::get_owned(&table, 1).unwrap();
    assert_eq!(base, Base { x: 1.0 });
}

#[test]
fn get_owned_sub1() {
    let table = Tables::from_str(EXAMPLE).unwrap();
    let sub1 = EntityTable::<Sub1Holder>::get_owned(&table, 2).unwrap();
    assert_eq!(
        sub1,
        Sub1 {
            base: Base { x: 1.0 },
            y1: 2.0
        }
    );
}

#[test]
fn get_owned_any() {
    let table = Tables::from_str(EXAMPLE).unwrap();

    // #1 = BASE(1.0);
    let any1 = EntityTable::<BaseAnyHolder>::get_owned(&table, 1).unwrap();
    assert_eq!(any1, BaseAny::Base(Box::new(Base { x: 1.0 })));

    // #2 = SUB_1(BASE((1.0)), 2.0);
    let any2 = EntityTable::<BaseAnyHolder>::get_owned(&table, 2).unwrap();
    assert_eq!(
        any2,
        BaseAny::Sub1(Box::new(Sub1 {
            base: Base { x: 1.0 },
            y1: 2.0
        }))
    );

    // #3 = SUB_2(#1, 4.0);
    let any3 = EntityTable::<BaseAnyHolder>::get_owned(&table, 3).unwrap();
    assert_eq!(
        any3,
        BaseAny::Sub2(Box::new(Sub2 {
            base: Base { x: 1.0 },
            y2: 4.0
        }))
    );
}
