//! Executable for espr EXPRESS language compiler

use espr::{ast::SyntaxTree, semantics::IR};
use quote::ToTokens;
use std::{fs, path::*};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Arguments {
    #[structopt(long = "num-error-lines", default_value = "10")]
    num_lines: usize,
    #[structopt(long = "check", help = "Check input EXPRESS definitions can be parsed")]
    check: bool,
    #[structopt(parse(from_os_str))]
    source: PathBuf,
}

fn main() {
    let args = Arguments::from_args();
    let src = fs::read_to_string(&args.source).expect("Failed to load EXPRESS source code");
    let st = match SyntaxTree::parse(&src) {
        Ok(st) => st,
        Err(e) => {
            for (code, kind) in e.errors {
                eprintln!(
                    "Syntax Error occurred while parsing following line [{:?}]:",
                    kind
                );
                for line in code.lines().take(args.num_lines) {
                    eprintln!("> {}", line);
                }
                eprintln!();
            }
            panic!("Syntax Error");
        }
    };
    if args.check {
        eprintln!("Parse succeeded");
        return;
    }

    let ir = IR::from_syntax_tree(&st).expect("Failed in semantic analysis phase");
    println!("#![allow(dead_code)]\n{}", ir.to_token_stream().to_string());
}
