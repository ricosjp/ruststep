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
          ARE,
          SORE,
          DORE
        );
      END_TYPE;

      ENTITY c_entity;
        x: REAL;
      END_ENTITY;

      TYPE c = c_entity;
      END_TYPE;

      ENTITY d;
        b: b;
      END_ENTITY;

	END_SCHEMA;
    "#
);

use test_schema::*;

const EXAMPLE: &str = r#"
DATA;
  #1 = A('KORE');
  #2 = C(C_ENTITY((1.0)));
  #3 = D(.SORE.);
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
fn deserialize_c_holder() {
    // from Record
    let (residual, p): (_, Record) = exchange::simple_record("C(C_ENTITY((1.0)))")
        .finish()
        .unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let c: CHolder = Deserialize::deserialize(&p).unwrap();
    assert_eq!(c, CHolder(PlaceHolder::Owned(CEntityHolder { x: 1.0 })));

    // from Parameter::Typed
    let (residual, p): (_, Parameter) = exchange::parameter("C((C_ENTITY((1.0))))")
        .finish()
        .unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let c: CHolder = Deserialize::deserialize(&p).unwrap();
    assert_eq!(c, CHolder(PlaceHolder::Owned(CEntityHolder { x: 1.0 })));

    // Test for PlaceHolder<AHolder>
    let (residual, p): (_, Record) = exchange::simple_record("C(C_ENTITY((1.0)))")
        .finish()
        .unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let c: PlaceHolder<CHolder> = Deserialize::deserialize(&p).unwrap();
    assert_eq!(
        c,
        PlaceHolder::Owned(CHolder(PlaceHolder::Owned(CEntityHolder { x: 1.0 })))
    );
}

#[test]
fn deserialize_d_holder() {
    // from Record
    let (residual, p): (_, Record) = exchange::simple_record("D(.SORE.)").finish().unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let d: DHolder = Deserialize::deserialize(&p).unwrap();
    assert_eq!(d, DHolder { b: B::Sore });

    // from Parameter::Typed
    let (residual, p): (_, Parameter) = exchange::parameter("D((.SORE.))").finish().unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let d: DHolder = Deserialize::deserialize(&p).unwrap();
    assert_eq!(d, DHolder { b: B::Sore });

    // Test for PlaceHolder<AHolder>
    let (residual, p): (_, Record) = exchange::simple_record("D(.SORE.)").finish().unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let d: PlaceHolder<DHolder> = Deserialize::deserialize(&p).unwrap();
    assert_eq!(d, PlaceHolder::Owned(DHolder { b: B::Sore }));
}

#[test]
fn get_owned() {
    let table = Tables::from_str(EXAMPLE).unwrap();
    let a = EntityTable::<AHolder>::get_owned(&table, 1).unwrap();
    assert_eq!(a, A("KORE".to_string()));
    let c = EntityTable::<CHolder>::get_owned(&table, 2).unwrap();
    assert_eq!(c, C(CEntity { x: 1.0 }));
    let d = EntityTable::<DHolder>::get_owned(&table, 3).unwrap();
    assert_eq!(d, D { b: B::Sore });
}
