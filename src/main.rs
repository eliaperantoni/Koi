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

    if false {
        println!("{:?}", lexer.collect::<Vec<token::Token>>());
    } else {
        let mut parser = parser::Parser::new(lexer);
        println!("{:?}", parser.parse());
    }

    Ok(())
}
