use std::fs;
use std::error::Error;

mod token;
mod lexer;
mod ast;
mod parser;

fn main() -> Result<(), Box<dyn Error>> {
    let source = fs::read_to_string("./prog")?;

    let lexer = lexer::Lexer::new(source);

    if false {
        println!("{:?}", lexer.collect::<Vec<token::Token>>());
    } else {
        let mut parser = parser::Parser::new(lexer);
        println!("{:?}", parser.parse_stmt());
    }

    Ok(())
}
