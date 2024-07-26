use crate::lexer::{Token, Tokens};
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

        let mut return_type = "()".to_string();

        //Parse ident
        let name = tokens.get().as_ident()?.to_owned();

        //Expect lparen
        tokens.get().as_lparen()?.to_owned();

        //Expect parameter list
        match tokens.get() {
            Token::Identifier(n) => {}
            Token::RParen => {}
            token => {}
        }

        //Parse optional return type
        if *tokens.peek() == Token::DoubleColon {
            tokens.add_pos(1);
            return_type = tokens.get().as_ident()?.to_owned();
        }

        tokens.get().as_lbrace()?;

        //TODO: handle inner

        tokens.get().as_rbrace()?;

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
        let name = tokens.get().as_ident()?.to_owned();
        tokens.get().as_double_colon()?;

        let mut r#type = tokens.get().as_ident()?.to_owned();

        if *tokens.try_peek()? == Token::Comma {
            tokens.add_pos(1);
        }

        Ok(Self { name, r#type })
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
