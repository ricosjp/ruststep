use nom::Finish;
use ruststep::{ast::*, parser::exchange, place_holder::*, tables::*};
use ruststep_derive::as_holder;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Table {
    base: HashMap<u64, as_holder!(Base)>,
    sub1: HashMap<u64, as_holder!(Sub1)>,
    sub2: HashMap<u64, as_holder!(Sub2)>,
}

impl Table {
    // ```
    // #1 = BASE(1.0);
    // #2 = SUB_1(BASE((1.0)), 2.0);
    // #3 = SUB_2(#1, 4.0);
    // ```
    fn example() -> Self {
        let mut table = Self::default();
        table.base.insert(1, BaseHolder { x: 1.0 });
        table.sub1.insert(
            2,
            Sub1Holder {
                base: BaseHolder { x: 1.0 }.into(),
                y1: 2.0,
            },
        );
        table.sub2.insert(
            3,
            Sub2Holder {
                base: RValue::Entity(1).into(),
                y2: 4.0,
            },
        );
        table
    }
}

#[derive(Clone, Debug, PartialEq, ruststep_derive::Holder)]
#[holder(table = Table)]
#[holder(field = base)]
#[holder(generate_deserialize)]
pub struct Base {
    x: f64,
}

#[derive(Clone, Debug, PartialEq, ruststep_derive::Holder)]
#[holder(table = Table)]
#[holder(field = sub1)]
#[holder(generate_deserialize)]
pub struct Sub1 {
    #[holder(use_place_holder)]
    base: Base,
    y1: f64,
}

#[derive(Clone, Debug, PartialEq, ruststep_derive::Holder)]
#[holder(table = Table)]
#[holder(field = sub2)]
#[holder(generate_deserialize)]
pub struct Sub2 {
    #[holder(use_place_holder)]
    base: Base,
    y2: f64,
}

#[derive(Clone, Debug, PartialEq, ruststep_derive::Holder)]
#[holder(table = Table)]
#[holder(generate_deserialize)]
enum BaseAny {
    #[holder(field = base)]
    #[holder(use_place_holder)]
    Base(Box<Base>),
    #[holder(field = sub1)]
    #[holder(use_place_holder)]
    Sub1(Box<Sub1>),
    #[holder(field = sub2)]
    #[holder(use_place_holder)]
    Sub2(Box<Sub2>),
}

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
        let table = Table::example();

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
        let table = Table::example();

        let holder = PlaceHolder::<BaseAnyHolder>::deserialize(&p).unwrap();
        dbg!(&holder);

        let owned = holder.into_owned(&table).unwrap();
        dbg!(&owned);
        assert_eq!(owned, answer);
    }
}
