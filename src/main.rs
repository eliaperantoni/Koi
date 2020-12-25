mod token;
mod lexer;
mod ast;
mod parser;

fn main() {
    let lexer = lexer::Lexer::new("[\"foo{1}bar\"]".to_owned());

    if false {
        println!("{:?}", lexer.collect::<Vec<token::Token>>());
    } else {
        let mut parser = parser::Parser::new(lexer);
        println!("{:?}", parser.parse_expression(0));
    }
}
