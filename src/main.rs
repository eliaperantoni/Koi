mod scanner;

use std::fs;
use std::env::args;

use scanner::Scanner;
use scanner::Token;

fn main() {
    let path = args().nth(1).unwrap_or(String::from("usage: ampere PATH"));

    let source = fs::read_to_string(path);
    let source = source.unwrap_or(String::from("could not read source"));

    let scanner= Scanner::new(&source);

    let tokens: Vec<Token> = scanner.collect();
}
