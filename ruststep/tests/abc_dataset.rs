//! Integration test for parsing STEP file downloaded from ABC Dataset

use nom::Finish;
use ruststep::parser;
use std::{fs, path::*};

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
