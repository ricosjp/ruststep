use std::{env, fs, io::Write, path::*};

#[test]
fn decode() {
    let code = include_str!("test.exp");
    let schemas = exp2rs::decode(code).unwrap();
    let out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("generated");
    if !out_dir.exists() {
        fs::create_dir_all(&out_dir).unwrap();
    }

    let mut file = std::fs::File::create(out_dir.join("decoded.rs")).unwrap();
    file.write(b"#![allow(dead_code)]\n").unwrap();
    for schema in schemas {
        file.write(schema.rust_code().as_bytes()).unwrap();
    }
}
