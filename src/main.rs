mod token;
mod lexer;

fn main() {
    let lexer = lexer::Lexer::new("\"foo{{}}baz\"".to_owned());
    println!("{:?}", lexer.collect::<Vec<token::Token>>());
}
