//! Integration test for parsing STEP file downloaded from ABC Dataset

use espr_runtime::parser;
use nom::Finish;
use std::{fs, path::*};

#[test]
fn abc_dataset_header() -> anyhow::Result<()> {
    let header = r#"
    HEADER;
    FILE_DESCRIPTION( ( '' ), ' ' );
    FILE_NAME( '/vol/tmp/translate-2747021839723325609/5ae2de121ced560fc658f4c5.step', '2018-04-27T08:23:47', ( '' ), ( '' ), ' ', ' ', ' ' );
    FILE_SCHEMA( ( 'AUTOMOTIVE_DESIGN { 1 0 10303 214 1 1 1 1 }' ) );
    ENDSEC;
    "#.trim();
    let exchange = parser::exchange::header_section(&header).finish().unwrap();
    dbg!(exchange);
    Ok(())
}

#[test]
fn abc_dataset_data() -> anyhow::Result<()> {
    let data = r#"
    DATA;
    #2 = PRODUCT_DEFINITION_CONTEXT( '', #57, 'design' );
    #73 =  ( NAMED_UNIT( #181 )PLANE_ANGLE_UNIT(  )SI_UNIT( $, .RADIAN. ) );
    ENDSEC;
    "#
    .trim();
    let exchange = parser::exchange::data_section(&data).finish().unwrap();
    dbg!(exchange);
    Ok(())
}

#[test]
fn abc_dataset() -> anyhow::Result<()> {
    let step_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/steps/00000050_80d90bfdd2e74e709956122a_step_000.step");
    let step_str = fs::read_to_string(step_file)?;
    let exchange = parser::exchange::exchange_file(&step_str).finish().unwrap();
    dbg!(exchange);
    Ok(())
}
