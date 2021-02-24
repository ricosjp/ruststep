//! Integration test for parsing STEP file downloaded from ABC Dataset

use espr_runtime::parser;
use nom::Finish;
use std::{fs, path::*};

#[test]
fn abc_dataset() -> anyhow::Result<()> {
    let step_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/steps/00000050_80d90bfdd2e74e709956122a_step_000.step");
    let step_str = fs::read_to_string(step_file)?;
    let exchange = parser::exchange::exchange_file(&step_str).finish().unwrap();
    dbg!(exchange);
    Ok(())
}
