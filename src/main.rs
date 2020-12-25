mod token;
mod lexer;
mod ast;
mod parser;

fn main() {
    let lexer = lexer::Lexer::new("{\"foo{2}bar\":1}".to_owned());
    let mut parser = parser::Parser::new(lexer);
    println!("{:?}", parser.parse_expression(0));
}
