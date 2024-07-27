use crate::generation::{CompileContext, Context, Module};
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::fs;

mod generation;
mod lexer;
mod parser;

fn main() {
    let file = fs::read_to_string("files/example.maga").expect("Failed to read file");

    let lexer = Lexer::new(&file);
    let mut tokens = lexer.tokens().expect("Failed to tokenize");
    println!("{:?}", tokens);

    let parser = Parser::new(&mut tokens);
    parser.print_ast();

    println!("\n\n\n");

    let mut context = CompileContext::new().unwrap();
    let mut module = context.context_mut().create_module("example").unwrap();
    context.generate(&mut module, &parser).unwrap();
}
