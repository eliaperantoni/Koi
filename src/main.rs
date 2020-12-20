mod token;
mod lexer;

fn main() {
    let lexer = lexer::Lexer::new("253.545&&".to_owned());
    println!("{:?}", lexer.collect::<Vec<token::Token>>());
}
