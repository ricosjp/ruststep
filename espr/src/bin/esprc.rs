//! Executable for espr EXPRESS language compiler

use espr::{parser::syntax_tree::SyntaxTree, semantics::IR};
use std::{fs, path::*};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Arguments {
    #[structopt(short = "l", long = "num_lines", default_value = "3")]
    num_lines: usize,
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
    let ir = IR::from_syntax_tree(&st).expect("Failed in semantic analysis phase");
    println!("{}", ir);
}
