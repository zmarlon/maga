use crate::parser::CompileError;
use logos::Logos;

#[derive(Logos, Clone, Debug, PartialEq, Eq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[token("let")]
    Let,
    #[token("var")]
    Var,

    #[token("fun")]
    Fun,
    #[token("unsafe")]
    Unsafe,
    #[token("const")]
    Const,

    #[token("return")]
    Return,
    #[token("if")]
    If,
    #[token("while")]
    While,
    #[token("break")]
    Break,
    #[token("continue")]
    Continue,
    #[token("loop")]
    Loop,

    #[token(":")]
    DoubleColon,
    #[token(";")]
    Semicolon,
    #[token(".")]
    Point,
    #[token(",")]
    Comma,

    #[token("=")]
    Assign,
    #[token("+")]
    Add,
    #[token("+=")]
    AddAssign,
    #[token("-")]
    Sub,
    #[token("-=")]
    SubAssign,
    #[token("*")]
    Mul,
    #[token("*=")]
    MulAssign,
    #[token("/")]
    Div,
    #[token("/=")]
    DivAssign,
    #[token("%")]
    Mod,
    #[token("%=")]
    ModAssign,

    //
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,

    #[token("<")]
    Less,
    #[token("<=")]
    LessEqual,
    #[token(">")]
    Greater,
    #[token(">=")]
    GreaterEqual,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_owned())]
    Identifier(String),

    #[regex("-(0|[1-9][0-9]*)", |lex| lex.slice().parse::<i64>().unwrap())]
    IntLiteral(i64),

    #[regex("(0|[1-9][0-9]*)", |lex| lex.slice().parse::<u64>().unwrap())]
    UIntLiteral(u64),
}

macro_rules! generate_as_fn {
    ($fn_name:ident, $variant:path) => {
        pub fn $fn_name(&self) -> Result<(), CompileError> {
            match self {
                $variant => Ok(()),
                token => Err(CompileError::new(format!(
                    "Expected {}, found {:?}",
                    stringify!($variant),
                    token
                ))),
            }
        }
    };
}

impl Token {
    pub fn as_ident(&self) -> Result<&str, CompileError> {
        match self {
            Token::Identifier(ident) => Ok(ident.as_str()),
            token => Err(CompileError::new(format!(
                "Expected ident, found {:?}",
                token
            ))),
        }
    }

    generate_as_fn!(as_lparen, Token::LParen);
    generate_as_fn!(as_rparen, Token::RParen);
    generate_as_fn!(as_lbrace, Token::LBrace);
    generate_as_fn!(as_rbrace, Token::RBrace);
    generate_as_fn!(as_double_colon, Token::DoubleColon);
    generate_as_fn!(as_comma, Token::Comma);
    generate_as_fn!(as_assign, Token::Assign);
    generate_as_fn!(as_semicolon, Token::Semicolon);
}

pub struct Lexer<'a> {
    lexer: logos::Lexer<'a, Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let lexer = Token::lexer(source);
        Self { lexer }
    }

    pub fn print_tokens(self) {
        for token in self.lexer {
            let Ok(token) = token else { continue };

            println!("{:?}", token);
        }
    }

    pub fn tokens(self) -> Result<Tokens, ()> {
        let tokens = self.lexer.into_iter().collect::<Result<Vec<Token>, ()>>()?;
        Ok(Tokens { tokens, pos: 0 })
    }
}

#[derive(Debug)]
pub struct Tokens {
    tokens: Vec<Token>,
    pos: usize,
}

impl Tokens {
    pub fn tokens(&self) -> &[Token] {
        &self.tokens
    }

    pub fn get_peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    pub fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    pub fn try_peek(&self) -> Result<&Token, CompileError> {
        if self.has_more() {
            Ok(self.peek())
        } else {
            Err(CompileError::new(
                "No more elements available in token stream".to_owned(),
            ))
        }
    }

    pub fn has_more(&self) -> bool {
        self.pos < self.tokens.len()
    }

    pub fn get(&mut self) -> &Token {
        if !self.has_more() {
            panic!("No more tokens in token stream")
        } else {
            self.pos += 1;
            &self.tokens[self.pos - 1]
        }
    }

    pub fn add_pos(&mut self, amount: usize) {
        self.pos += amount
    }
}
