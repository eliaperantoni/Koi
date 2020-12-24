mod token;
mod lexer;
mod ast;
mod parser;

fn main() {
    let lexer = lexer::Lexer::new("5++".to_owned());
    let mut parser = parser::Parser::new(lexer);
    println!("{:?}", parser.parse_expression(0));
}
