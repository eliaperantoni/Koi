mod token;
mod lexer;

fn main() {
    let lexer = lexer::Lexer::new("ls -l cd ..".to_owned());
    println!("{:?}", lexer.collect::<Vec<token::Token>>());
}
