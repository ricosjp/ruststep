//! Integration test for parsing STEP file downloaded from ABC Dataset

use espr_runtime::parser;
use nom::Finish;
use std::{fs, path::*};

#[test]
fn header() -> anyhow::Result<()> {
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

fn format_example() -> anyhow::Result<String> {
    let step_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/steps/00000050_80d90bfdd2e74e709956122a_step_000.step");
    let step_str = fs::read_to_string(step_file)?;
    Ok(step_str)
}

#[test]
fn abc_dataset_data_line() -> anyhow::Result<()> {
    let mut failed = false;
    for line in format_example()?
        .lines()
        .skip(7 /* lines before DATA; */)
        .take(1671 /* lines in DATA secition */)
    {
        match parser::exchange::entity_instance(line).finish() {
            Ok(_) => continue,
            Err(err) => {
                dbg!(line);
                dbg!(err);
                failed = true;
            }
        }
    }
    if failed {
        anyhow::bail!("Parse failed");
    }
    Ok(())
}

#[test]
fn abc_dataset() -> anyhow::Result<()> {
    let step_str = format_example()?;
    let exchange = parser::exchange::exchange_file(&step_str).finish().unwrap();
    dbg!(exchange);
    Ok(())
}
