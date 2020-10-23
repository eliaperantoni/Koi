mod scanner;

use std::fs;
use std::env::args;

use scanner::Scanner;

fn main() {
    let path = args().nth(1).unwrap_or(String::from("usage: ampere PATH"));

    let source = fs::read_to_string(path);
    let source = source.unwrap_or(String::from("could not read source"));

    let scanner= Scanner::new(&source);

    for token in scanner {
        println!("{:?}", token);
    }
}
