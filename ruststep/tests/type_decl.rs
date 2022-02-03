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

      TYPE a_rename = a;
      END_TYPE;

      ENTITY a_entity;
        a: a;
      END_ENTITY;

      TYPE b = ENUMERATION OF (
          ARE,
          SORE,
          DORE
        );
      END_TYPE;

      TYPE b_rename = b;
      END_TYPE;

      TYPE b_rename_rename = b_rename;
      END_TYPE;

      ENTITY b_entity;
        b: b;
      END_ENTITY;

      ENTITY abs_entity;
        x: REAL;
      END_ENTITY;

      TYPE c = abs_entity;
      END_TYPE;

    END_SCHEMA;
    "#
);

use test_schema::*;

const EXAMPLE: &str = r#"
DATA;
  #1 = A('KORE');
  #2 = A_RENAME(#1);
  #3 = A_ENTITY(#1);
  #4 = B_RENAME(.SORE.);
  #5 = B_RENAME_RENAME(#4);
  #6 = B_ENTITY(.DORE.);
  #7 = C(ABS_ENTITY((1.0)));
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
fn deserialize_a_rename_holder() {
    // from Record
    let (residual, p): (_, Record) = exchange::simple_record("A_RENAME(A(('KORE')))")
        .finish()
        .unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let a: ARenameHolder = Deserialize::deserialize(&p).unwrap();
    assert_eq!(
        a,
        ARenameHolder(PlaceHolder::Owned(AHolder("KORE".to_string())))
    );

    // from Parameter::Typed
    let (residual, p): (_, Parameter) = exchange::parameter("A_RENAME((A(('KORE'))))")
        .finish()
        .unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let a: ARenameHolder = Deserialize::deserialize(&p).unwrap();
    assert_eq!(
        a,
        ARenameHolder(PlaceHolder::Owned(AHolder("KORE".to_string())))
    );

    // Test for PlaceHolder<AHolder>
    let (residual, p): (_, Record) = exchange::simple_record("A_RENAME(A(('KORE')))")
        .finish()
        .unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let a: PlaceHolder<ARenameHolder> = Deserialize::deserialize(&p).unwrap();
    assert_eq!(
        a,
        PlaceHolder::Owned(ARenameHolder(PlaceHolder::Owned(AHolder(
            "KORE".to_string()
        ))))
    );
}

#[test]
fn deserialize_a_entity_holder() {
    // from Record
    let (residual, p): (_, Record) = exchange::simple_record("A_ENTITY(A(('KORE')))")
        .finish()
        .unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let a: AEntityHolder = Deserialize::deserialize(&p).unwrap();
    assert_eq!(
        a,
        AEntityHolder {
            a: PlaceHolder::Owned(AHolder("KORE".to_string()))
        }
    );

    // from Parameter::Typed
    let (residual, p): (_, Parameter) = exchange::parameter("A_ENTITY((A(('KORE'))))")
        .finish()
        .unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let a: AEntityHolder = Deserialize::deserialize(&p).unwrap();
    assert_eq!(
        a,
        AEntityHolder {
            a: PlaceHolder::Owned(AHolder("KORE".to_string()))
        }
    );

    // Test for PlaceHolder<AHolder>
    let (residual, p): (_, Record) = exchange::simple_record("A_ENTITY(A(('KORE')))")
        .finish()
        .unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let a: PlaceHolder<AEntityHolder> = Deserialize::deserialize(&p).unwrap();
    assert_eq!(
        a,
        PlaceHolder::Owned(AEntityHolder {
            a: PlaceHolder::Owned(AHolder("KORE".to_string()))
        })
    );
}

#[test]
fn deserialize_b_rename_holder() {
    // from Record
    let (residual, p): (_, Record) = exchange::simple_record("B_RENAME(.ARE.)").finish().unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let b: BRenameHolder = Deserialize::deserialize(&p).unwrap();
    assert_eq!(b, BRenameHolder(B::Are));

    // from Parameter::Typed
    let (residual, p): (_, Parameter) = exchange::parameter("B_RENAME((.ARE.))").finish().unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let b: BRenameHolder = Deserialize::deserialize(&p).unwrap();
    assert_eq!(b, BRenameHolder(B::Are));

    // Test for PlaceHolder<AHolder>
    let (residual, p): (_, Record) = exchange::simple_record("B_RENAME(.ARE.)").finish().unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let b: PlaceHolder<BRenameHolder> = Deserialize::deserialize(&p).unwrap();
    assert_eq!(b, PlaceHolder::Owned(BRenameHolder(B::Are)));
}

#[test]
fn deserialize_b_rename_rename_holder() {
    // from Record
    let (residual, p): (_, Record) = exchange::simple_record("B_RENAME_RENAME(B_RENAME((.ARE.)))")
        .finish()
        .unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let b: BRenameRenameHolder = Deserialize::deserialize(&p).unwrap();
    assert_eq!(
        b,
        BRenameRenameHolder(PlaceHolder::Owned(BRenameHolder(B::Are)))
    );

    // from Parameter::Typed
    let (residual, p): (_, Parameter) = exchange::parameter("B_RENAME_RENAME((B_RENAME((.ARE.))))")
        .finish()
        .unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let b: BRenameRenameHolder = Deserialize::deserialize(&p).unwrap();
    assert_eq!(
        b,
        BRenameRenameHolder(PlaceHolder::Owned(BRenameHolder(B::Are)))
    );

    // Test for PlaceHolder<AHolder>
    let (residual, p): (_, Record) = exchange::simple_record("B_RENAME_RENAME(B_RENAME((.ARE.)))")
        .finish()
        .unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let b: PlaceHolder<BRenameRenameHolder> = Deserialize::deserialize(&p).unwrap();
    assert_eq!(
        b,
        PlaceHolder::Owned(BRenameRenameHolder(PlaceHolder::Owned(BRenameHolder(
            B::Are
        ))))
    );
}

#[test]
fn deserialize_b_entity_holder() {
    // from Record
    let (residual, p): (_, Record) = exchange::simple_record("B_ENTITY(.ARE.)").finish().unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let b: BEntityHolder = Deserialize::deserialize(&p).unwrap();
    assert_eq!(b, BEntityHolder { b: B::Are });

    // from Parameter::Typed
    let (residual, p): (_, Parameter) = exchange::parameter("B_ENTITY((.ARE.))").finish().unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let b: BEntityHolder = Deserialize::deserialize(&p).unwrap();
    assert_eq!(b, BEntityHolder { b: B::Are });

    // Test for PlaceHolder<AHolder>
    let (residual, p): (_, Record) = exchange::simple_record("B_ENTITY(.ARE.)").finish().unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let b: PlaceHolder<BEntityHolder> = Deserialize::deserialize(&p).unwrap();
    assert_eq!(b, PlaceHolder::Owned(BEntityHolder { b: B::Are }));
}

#[test]
fn deserialize_c_holder() {
    // from Record
    let (residual, p): (_, Record) = exchange::simple_record("C(ABS_ENTITY((1.0)))")
        .finish()
        .unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let c: CHolder = Deserialize::deserialize(&p).unwrap();
    assert_eq!(c, CHolder(PlaceHolder::Owned(AbsEntityHolder { x: 1.0 })));

    // from Parameter::Typed
    let (residual, p): (_, Parameter) = exchange::parameter("C((ABS_ENTITY((1.0))))")
        .finish()
        .unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let c: CHolder = Deserialize::deserialize(&p).unwrap();
    assert_eq!(c, CHolder(PlaceHolder::Owned(AbsEntityHolder { x: 1.0 })));

    // Test for PlaceHolder<AHolder>
    let (residual, p): (_, Record) = exchange::simple_record("C(ABS_ENTITY((1.0)))")
        .finish()
        .unwrap();
    assert_eq!(residual, "");
    dbg!(&p);
    let c: PlaceHolder<CHolder> = Deserialize::deserialize(&p).unwrap();
    assert_eq!(
        c,
        PlaceHolder::Owned(CHolder(PlaceHolder::Owned(AbsEntityHolder { x: 1.0 })))
    );
}

#[test]
fn get_owned() {
    let table = Tables::from_str(EXAMPLE).unwrap();
    let a = EntityTable::<AHolder>::get_owned(&table, 1).unwrap();
    assert_eq!(a, A("KORE".to_string()));
    let a = EntityTable::<ARenameHolder>::get_owned(&table, 2).unwrap();
    assert_eq!(a, ARename(A("KORE".to_string())));
    let a = EntityTable::<AEntityHolder>::get_owned(&table, 3).unwrap();
    assert_eq!(
        a,
        AEntity {
            a: A("KORE".to_string())
        }
    );
    let b = EntityTable::<BRenameHolder>::get_owned(&table, 4).unwrap();
    assert_eq!(b, BRename(B::Sore));
    let b = EntityTable::<BRenameRenameHolder>::get_owned(&table, 5).unwrap();
    assert_eq!(b, BRenameRename(BRename(B::Sore)));
    let b = EntityTable::<BEntityHolder>::get_owned(&table, 6).unwrap();
    assert_eq!(b, BEntity { b: B::Dore });
    let c = EntityTable::<CHolder>::get_owned(&table, 7).unwrap();
    assert_eq!(c, C(AbsEntity { x: 1.0 }));
}
