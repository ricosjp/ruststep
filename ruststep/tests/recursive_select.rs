use ruststep::tables::*;
use std::str::FromStr;

espr_derive::inline_express!(
    r#"	
    SCHEMA test_schema;
      ENTITY a;
        x: REAL;
      END_ENTITY;

      ENTITY b;
        y: INTEGER;
      END_ENTITY;

      ENTITY c;
        z: STRING;
      END_ENTITY;

      TYPE sup = SELECT (a, b);
      END_TYPE;

      TYPE sup_sup = SELECT (sup, c);
      END_TYPE;
    END_SCHEMA;
    "#
);

use test_schema::*;

const EXAMPLE: &str = r#"
DATA;
  #1 = A(3.34);
  #2 = B(57);
  #3 = C('hoge');
ENDSEC;
"#;

#[test]
fn get_owned_sup() {
    let table = Tables::from_str(EXAMPLE).unwrap();
    let sup1 = EntityTable::<SupHolder>::get_owned(&table, 1).unwrap();
    assert_eq!(sup1, Sup::A(Box::new(A { x: 3.34 })));
    let sup2 = EntityTable::<SupHolder>::get_owned(&table, 2).unwrap();
    assert_eq!(sup2, Sup::B(Box::new(B { y: 57 })));
}

#[test]
fn get_owned_supsup() {
    let table = Tables::from_str(EXAMPLE).unwrap();
    let sup1 = EntityTable::<SupHolder>::get_owned(&table, 1).unwrap();
    let supsup1 = EntityTable::<SupSupHolder>::get_owned(&table, 1).unwrap();
    assert_eq!(supsup1, SupSup::Sup(Box::new(sup1)));
    let sup2 = EntityTable::<SupHolder>::get_owned(&table, 2).unwrap();
    let supsup2 = EntityTable::<SupSupHolder>::get_owned(&table, 2).unwrap();
    assert_eq!(supsup2, SupSup::Sup(Box::new(sup2)));
    let supsup3 = EntityTable::<SupSupHolder>::get_owned(&table, 3).unwrap();
    assert_eq!(supsup3, SupSup::C(Box::new(C { z: "hoge".into() })));
}
