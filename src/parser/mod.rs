use crate::lexer::{Token, Tokens};
use std::ops::{Deref, DerefMut};

mod function;
mod r#type;
pub use function::*;
pub use r#type::*;

#[derive(Debug)]
pub struct CompileError {
    message: String,
}

impl CompileError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

pub trait FromTokenStream {
    fn from_token_stream(tokens: &mut Tokens) -> Result<Self, CompileError>
    where
        Self: Sized;
}

#[derive(Debug)]
pub enum Element {
    SourceFile(Vec<Element>),
    Function(Function),
}

#[derive(Debug, Default)]
pub struct SourceFile(pub Vec<Element>);

impl Deref for SourceFile {
    type Target = Vec<Element>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SourceFile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub enum ParsingState {
    Idle,
}

pub struct Parser {
    root: SourceFile,
}

impl Parser {
    pub fn new(tokens: &mut Tokens) -> Self {
        let mut root = SourceFile::default();
        let mut parsing_state = ParsingState::Idle;

        while tokens.has_more() {
            let token = tokens.peek();
            match token {
                Token::Fun => {
                    let function = Function::from_token_stream(tokens).unwrap(); //TODO:
                    (*root).push(Element::Function(function));
                }
                token => {
                    println!("Suspicious token : {:?}", token);
                }
            }
        }

        Self { root }
    }

    pub fn print_ast(&self) {
        println!("{:?}", &self.root);
    }

    pub fn root(&self) -> &SourceFile {
        &self.root
    }
}
