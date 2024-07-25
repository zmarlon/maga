use crate::lexer::{Token, Tokens};
use std::cmp::PartialEq;
use std::ops::{Deref, DerefMut};

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
pub struct SourceFile(Vec<Element>);

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

#[derive(Debug)]
pub struct Function {
    name: String,
    return_type: String,

    params: Vec<FunctionParam>,

    body: Vec<Element>,
}

impl FromTokenStream for Function {
    fn from_token_stream(tokens: &mut Tokens) -> Result<Self, CompileError> {
        tokens.add_pos(1);

        let mut name = String::default();
        let mut return_type = "()".to_string();

        //Parse ident
        match tokens.get() {
            Token::Identifier(ident) => {
                name = ident.clone();
            }
            token => {
                return Err(CompileError::new(format!(
                    "Expected identifier, found {:?} instead",
                    token
                )));
            }
        }

        //Expect lparen
        match tokens.get() {
            Token::LParen => {}
            token => {
                return Err(CompileError::new(format!(
                    "Expected LParen, found {:?} instead",
                    token
                )));
            }
        }

        //Expect parameter list
        match tokens.get() {
            Token::Identifier(n) => {}
            Token::RParen => {}
            token => {}
        }

        //Parse optional return type
        if *tokens.peek() == Token::DoubleColon {
            tokens.add_pos(1);
            match tokens.get() {
                Token::Identifier(ident) => {
                    return_type = ident.clone();
                }
                token => {
                    return Err(CompileError::new(format!(
                        "Expected return type, found {:?} instead",
                        token
                    )));
                }
            }
        }

        match tokens.get() {
            Token::LBrace => {
                //TODO: parse function scope
            }
            token => {
                return Err(CompileError::new(format!(
                    "Expected  function scope, found {:?} instead",
                    token
                )));
            }
        }

        match tokens.get() {
            Token::RBrace => {}
            _ => {}
        }

        Ok(Self {
            name,
            return_type,
            params: vec![],
            body: vec![],
        })
    }
}

#[derive(Debug)]
pub struct FunctionParam {
    name: String,
    r#type: String,
}

impl FromTokenStream for FunctionParam {
    fn from_token_stream(tokens: &mut Tokens) -> Result<Self, CompileError>
    where
        Self: Sized,
    {
        todo!()
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
                _ => {}
            }
        }

        Self { root }
    }

    pub fn print_ast(&self) {
        println!("{:?}", &self.root);
    }
}
