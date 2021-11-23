use espr::{ast::SyntaxTree, codegen::rust::*, ir::IR};

use std::{
    fs,
    io::Write,
    path::*,
    process::{Command, Stdio},
};

fn rustfmt(tt: String) -> String {
    let mut child = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn rustfmt process");
    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin
            .write_all(tt.as_bytes())
            .expect("Failed to write to stdin");
    });
    let output = child.wait_with_output().expect("Failed to read stdout");
    String::from_utf8(output.stdout).expect("Non UTF-8 string found")
}

fn test(exp: &Path, rs: &Path) {
    let input = fs::read_to_string(exp).unwrap();

    let st = SyntaxTree::parse(&input).expect("Tokenize failed");
    let ir = IR::from_syntax_tree(&st).expect("Failed in semantic analysis phase");
    let tt = ir.to_token_stream(CratePrefix::External).to_string();

    let tt = rustfmt(tt);

    let mut generated = PathBuf::from(rs);
    generated.set_extension("generated.rs");
    fs::write(generated, tt).unwrap();

    // TODO compare rs and generated
}

#[test]
fn try_generate() {
    let espr_root = PathBuf::from(std::env!("CARGO_MANIFEST_DIR"));
    let test_case_root = espr_root.join("tests/cases");
    test(
        &test_case_root.join("simple.exp"),
        &test_case_root.join("simple.rs"),
    );
}
