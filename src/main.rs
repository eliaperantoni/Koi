#![feature(or_patterns)]
#![feature(with_options)]
#![feature(option_insert)]

use std::fs;
use std::error::Error;

mod token;
mod lexer;
mod ast;
mod parser;
mod interp;

fn main() -> Result<(), Box<dyn Error>> {
    let source = fs::read_to_string("./prog.amp")?;

    let lexer = lexer::new(source);

    match 2 {
        1 => {
            println!("{:?}", lexer.collect::<Vec<token::Token>>());
        }
        2 => {
            let mut parser = parser::Parser::new(lexer);
            println!("{:?}", parser.parse());
        }
        3 => {
            let mut parser = parser::Parser::new(lexer);
            let prog = parser.parse();

            let mut interpreter = interp::Interpreter::new();
            interpreter.run(prog);
        }
        _ => unreachable!()
    }

    Ok(())
}
