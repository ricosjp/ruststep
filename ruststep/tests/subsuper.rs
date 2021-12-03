use nom::Finish;
use ruststep::{ast::*, parser::exchange, place_holder::*, tables::*};
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

const EXAMPLE: &str = r#"
DATA;
  #1 = BASE(1.0);
  #2 = SUB(BASE((1.0)), 2.0);
  #3 = SUBSUB(#2, 4.0);
ENDSEC;
"#;
