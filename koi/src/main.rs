use std::env;
use std::fs;
use std::io;
use std::io::Read;

use clap::{App, Arg};
use itertools::Itertools;
use linefeed::{Interface, ReadResult};

use koi_core::lexer::new as new_lexer;
use koi_core::ast;
use koi_core::parser;
use koi_core::interp;

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

    let source: Option<String> = if matches.is_present("stdin") {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap();
        Some(buffer)
    } else if matches.is_present("path") {
        Some(fs::read_to_string(matches.value_of("path").unwrap_or("Koifile")).unwrap())
    } else {
        None
    };

    match source {
        Some(source) => {
            let (mut interpreter, prog) = interpreter(source);

            interpreter.set_args(script_args);
            interpreter.set_root(matches.value_of("path").unwrap());

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
        },
        None => {
            loop {
                let mut reader = Interface::new("koi").unwrap();

                reader.set_prompt("koi >> ").unwrap();

                let mut interpreter = interp::Interpreter::new();
                interpreter.do_collect();

                while let ReadResult::Input(input) = reader.read_line().unwrap() {
                    if input == "exit" {
                        println!("Exiting...");

                        std::process::exit(0);
                    }

                    let lexer = new_lexer(input);

                    let mut parser = parser::Parser::new(lexer);
                    let prog = parser.parse();

                    interpreter.run(prog);

                    if let Some(output_buffer) = interpreter.collector {
                        print!("{}", output_buffer);
                        interpreter.collector = None
                    }

                    interpreter.collector = None
                }
            }
        }
    };
}

fn interpreter(source: String) -> (interp::Interpreter, ast::Prog) {
    let lexer = new_lexer(source);

    let mut parser = parser::Parser::new(lexer);
    let prog = parser.parse();

    (interp::Interpreter::new(), prog)
}
