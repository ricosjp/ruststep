use espr::{parser::*, semantics::*};
use quote::*;
use std::{env, fs, io::Write, path::*};

#[test]
fn decode() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // load EXPRESS
    let code = fs::read_to_string(root.join("express/test.exp")).unwrap();
    let st = SyntaxTree::parse(&code).unwrap();
    dbg!(&st);
    let ns = Namespace::new(&st).unwrap();
    dbg!(&ns);
    let ir = IR::legalize(&ns, &Scope::root(), &st).unwrap();
    dbg!(&ir);

    // Generate Rust code
    let out_dir = root.join("generated");
    if !out_dir.exists() {
        fs::create_dir_all(&out_dir).unwrap();
    }
    let dest = out_dir.join("decoded.rs");
    let mut file = std::fs::File::create(&dest).unwrap();
    file.write(b"#![allow(dead_code)]\n").unwrap();
    file.write(ir.to_token_stream().to_string().as_bytes())
        .unwrap();
    file.write(b"fn main() {}\n").unwrap();

    // Test the generate Rust code is compile-able
    let t = trybuild::TestCases::new();
    t.pass(dest);
}
