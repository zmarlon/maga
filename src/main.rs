use crate::lexer::Lexer;
use crate::parser::Parser;
use std::fs;

mod lexer;
mod parser;

fn main() {
    let file = fs::read_to_string("files/example.maga").expect("Failed to read file");

    let lexer = Lexer::new(&file);
    let mut tokens = lexer.tokens().expect("Failed to tokenize");
    println!("{:?}", tokens);

    let parser = Parser::new(&mut tokens);
    parser.print_ast();
}
