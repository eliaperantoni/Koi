use std::fs;
use std::env::args;e


fn main() {
    let path = args().nth(1).unwrap_or(String::from("Usage: amp PATH"));

    let source = fs::read_to_string(path);
    let source = source.unwrap_or(String::from("Could not read source"));

    println!("{}", source);
}
