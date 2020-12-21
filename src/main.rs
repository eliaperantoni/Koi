mod token;
mod lexer;

fn main() {
    let lexer = lexer::Lexer::new("while for".to_owned());
    println!("{:?}", lexer.collect::<Vec<token::Token>>());
}
