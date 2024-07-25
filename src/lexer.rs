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

    #[regex("[a-zA-Z0-9]+", |lex| lex.slice().to_owned())]
    Identifier(String),
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