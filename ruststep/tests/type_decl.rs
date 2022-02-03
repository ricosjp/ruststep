// Test for deserializing Holder structs

use ruststep::tables::*;
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
  #3 = C(#1);
  #4 = D(.DORE.);
  #5 = E(#1, .SORE., #3, #4);
ENDSEC;
"#;

#[test]
fn get_owned() {
    let table = Tables::from_str(EXAMPLE).unwrap();
    let a = EntityTable::<AHolder>::get_owned(&table, 1).unwrap();
    assert_eq!(a, A("KORE".to_string()));

    let c = EntityTable::<CHolder>::get_owned(&table, 3).unwrap();
    assert_eq!(c, C(A("KORE".to_string())));

    let d = EntityTable::<DHolder>::get_owned(&table, 4).unwrap();
    assert_eq!(d, D(B::Dore));

    let e = EntityTable::<EHolder>::get_owned(&table, 5).unwrap();
    assert_eq!(
        e,
        E {
            a: A("KORE".to_string()),
            b: B::Sore,
            c: C(A("KORE".to_string())),
            d: D(B::Dore),
        }
    );
}
