#![feature(or_patterns)]
#![feature(with_options)]
#![feature(option_insert)]
#![feature(test)]

use std::fs;
use std::io;
use std::io::Read;

use clap::{App, Arg};

use crate::lexer::new as new_lexer;

mod token;
mod lexer;
mod ast;
mod parser;
mod interp;

fn main() {
    let matches = App::new("Koi")
        .version("1.0.0")
        .author("Elia Perantoni <perantonielia0@gmail.com>")
        .arg(Arg::with_name("PATH").help("Path to soure file. If omitted will read from STDIN."))
        .get_matches();

    let source = match matches.value_of("PATH") {
        Some(path) => fs::read_to_string(path).unwrap(),
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer).unwrap();
            buffer
        }
    };

    let lexer = new_lexer(source);

    let mut parser = parser::Parser::new(lexer);
    let prog = parser.parse();

    let mut interpreter = interp::Interpreter::new();
    interpreter.run(prog);
}
