mod token;
mod lexer;
mod ast;

fn main() {
    let lexer = lexer::Lexer::new("ls -l cd ..".to_owned());
    println!("{:?}", lexer.collect::<Vec<token::Token>>());
}
