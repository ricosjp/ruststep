#![cfg(feature = "ap201")]

use ruststep::{ap201::explicit_draughting::*, tables::*};
use std::str::FromStr;

fn example() -> Tables {
    Tables::from_str(
        r#"
        DATA;
          #1 = COLOUR_RGB(1.0, 0.0, 0.0);
        ENDSEC;
        "#,
    )
    .unwrap()
}

#[test]
fn get_owned() {
    let tables = example();
    let colour = EntityTable::<ColourRgbHolder>::get_owned(&tables, 1).unwrap();
    dbg!(colour);
}

#[test]
fn get_owned_any() {
    let tables = example();
    let colour = EntityTable::<ColourSpecificationAnyHolder>::get_owned(&tables, 1).unwrap();
    dbg!(colour);
}
