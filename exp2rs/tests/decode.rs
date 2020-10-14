extern crate exp2rs;
use std::io::Write;

#[test]
fn decode() {
    let code = include_str!("test.exp");
    let schemas = exp2rs::decode(code).unwrap();

    let mut file = std::fs::File::create("tests/decoded.rs").unwrap();
    file.write(b"#![allow(dead_code)]\n").unwrap();
    for schema in schemas {
        file.write(schema.rust_code().as_bytes()).unwrap();
    }
}
