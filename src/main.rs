#![feature(test)]

use std::env;
use std::fs;
use std::io;
use std::io::Read;
use std::path::PathBuf;

use clap::{App, Arg};
use itertools::Itertools;

use crate::lexer::new as new_lexer;

mod token;
mod lexer;
mod ast;
mod parser;
mod interp;

fn split_args() -> (Vec<String>, Vec<String>) {
    let args = env::args().collect_vec();

    if let Some(i) = args.iter().position(|arg| arg == "--") {
        (args[..i].to_vec(), args[i + 1..].to_vec())
    } else {
        (args, vec![])
    }
}

fn main() {
    let (koi_args, script_args) = split_args();

    let matches = App::new("Koi")
        .version("1.0.0")
        .author("Elia Perantoni <perantonielia0@gmail.com>")
        .arg(
            Arg::with_name("path")
                .value_name("PATH")
                .index(1)
                .takes_value(true)
                .help("Path to source file.")
        )
        .arg(
            Arg::with_name("stdin")
                .short("s")
                .long("stdin")
                .takes_value(false)
                .help("Read script from stdin.")
                .conflicts_with("path")
        )
        .arg(
            Arg::with_name("fn")
                .short("f")
                .long("--fn")
                .takes_value(true)
                .help("Function to call.")
        )
        .get_matches_from(koi_args);

    let source = if matches.is_present("stdin") {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap();
        buffer
    } else {
        fs::read_to_string(matches.value_of("path").unwrap_or("Koifile")).unwrap()
    };

    let lexer = new_lexer(source);

    let mut parser = parser::Parser::new(lexer);
    let prog = parser.parse();

    let mut interpreter = interp::Interpreter::new();
    interpreter.set_args(script_args);
    if let Some(path) = matches.value_of("path") {
        let mut import_root = std::fs::canonicalize(PathBuf::from(path))
            .expect("couldn't set import root");
        import_root.pop();
        interpreter.set_import_root(import_root);
    }
    interpreter.run(prog);

    if let Some(f) = matches.value_of("fn") {
        use ast::{Stmt, Expr};

        interpreter.run(vec![
            Stmt::Expr(Expr::Call {
                func: Box::new(Expr::Get(f.to_string())),
                args: vec![],
            })
        ]);
    }
}
