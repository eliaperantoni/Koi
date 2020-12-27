mod token;
mod lexer;
mod ast;
mod parser;

fn main() {
    let lexer = lexer::Lexer::new("\"/usr/bin/bash\" -c input.txt | {\"output.txt\".uppercase()}".to_owned());

    if false {
        println!("{:?}", lexer.collect::<Vec<token::Token>>());
    } else {
        let mut parser = parser::Parser::new(lexer);
        println!("{:?}", parser.parse_stmt());
    }
}
