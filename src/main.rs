mod tokenizer;
use regex::Regex;

use crate::tokenizer::WHITESPACE_REGEX;

fn main() {
    println!("Hello, world!");
    let source_code = "   ";
    let whitespeace_regex = Regex::new(WHITESPACE_REGEX).unwrap();
    let is_whitespace = whitespeace_regex.is_match(source_code);
    println!("{}", is_whitespace);
}
