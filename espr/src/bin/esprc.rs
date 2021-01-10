//! Executable for espr EXPRESS language compiler

use espr::{parser::syntax_tree::SyntaxTree, semantics::IR};
use std::{fs, path::*};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Arguments {
    #[structopt(parse(from_os_str))]
    source: PathBuf,
}

fn main() {
    let args = Arguments::from_args();
    let src = fs::read_to_string(&args.source).expect("Failed to load EXPRESS source code");
    let st = SyntaxTree::parse(&src).expect("Failed to parse input");
    let ir = IR::from_syntax_tree(&st).expect("Failed in semantic analysis phase");
    println!("{}", ir);
}
