use crate::lexer::{Token, Tokens};
use crate::parser::{CompileError, FromTokenStream, LiteralType};
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Operator {
    Add,
    AddAssign,

    Sub,
    SubAssign,

    Mul,
    MulAssign,

    Div,
    DivAssign,

    Mod,
    ModAssign,

    Less,
    LessEqual,

    Greater,
    GreaterEqual,
}

impl Operator {}

impl Operator {
    pub fn from_token(token: &Token) -> Result<Self, CompileError> {
        match token {
            Token::Add => Ok(Operator::Add),
            Token::AddAssign => Ok(Operator::AddAssign),
            Token::Sub => Ok(Operator::Sub),
            Token::SubAssign => Ok(Operator::SubAssign),
            Token::Mul => Ok(Operator::Mul),
            Token::MulAssign => Ok(Operator::MulAssign),
            Token::Div => Ok(Operator::Div),
            Token::DivAssign => Ok(Operator::DivAssign),
            Token::Mod => Ok(Operator::Mod),
            Token::ModAssign => Ok(Operator::ModAssign),
            Token::Less => Ok(Operator::Less),
            Token::LessEqual => Ok(Operator::LessEqual),
            Token::Greater => Ok(Operator::Greater),
            Token::GreaterEqual => Ok(Operator::GreaterEqual),
            token => Err(CompileError::new(format!(
                "Expected operator, found {:?}",
                token
            ))),
        }
    }
}

impl FromTokenStream for Operator {
    fn from_token_stream(tokens: &mut Tokens) -> Result<Self, CompileError>
    where
        Self: Sized,
    {
        Self::from_token(tokens.get())
    }
}

#[derive(Debug)]
pub enum Expression {
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    Constant(ConstantExpression),
    Variable(VariableExpression),
    Call(CallExpression),
}

#[derive(Debug)]
pub struct BinaryExpression {
    operator: Operator,
    lhs: Box<Expression>,
    rhs: Box<Expression>,
}

#[derive(Debug)]
pub struct UnaryExpression {
    operator: Operator,
    lhs: Box<Expression>,
}

#[derive(Debug)]
pub struct ConstantExpression {
    pub value: ConstantExpressionValue,
}

#[derive(Debug)]
pub enum ConstantExpressionValue {
    Int(i64),
    UInt(u64),
}

impl FromTokenStream for ConstantExpression {
    fn from_token_stream(tokens: &mut Tokens) -> Result<Self, CompileError>
    where
        Self: Sized,
    {
        match tokens.get() {
            Token::UIntLiteral(lit) => Ok(Self {
                value: ConstantExpressionValue::UInt(*lit),
            }),
            Token::IntLiteral(lit) => Ok(Self {
                value: ConstantExpressionValue::Int(*lit),
            }),
            token => Err(CompileError::new(format!(
                "Invalid constant type: {:?}",
                token
            ))),
        }
    }
}

#[derive(Debug)]
pub struct VariableExpression {
    pub name: String,
}

#[derive(Debug)]
pub struct CallExpression {
    name: String,
    args: Vec<Expression>,
}

impl Expression {
    fn parse_ident(tokens: &mut Tokens) -> Result<Self, CompileError> {
        let name = tokens.get().as_ident()?.to_owned();

        if *tokens.peek() != Token::LParen {
            Ok(Self::Variable(VariableExpression { name }))
        } else {
            tokens.get().as_lparen()?;

            let mut args = Vec::new();

            while *tokens.peek() != Token::RParen {
                args.push(Self::parse(tokens)?);

                if *tokens.peek() == Token::RParen {
                    break;
                }

                tokens.get().as_comma()?;
            }

            tokens.get().as_rparen()?;

            Ok(Self::Call(CallExpression { name, args }))
        }
    }

    fn parse_constant(tokens: &mut Tokens) -> Result<Self, CompileError> {
        Ok(Expression::Constant(ConstantExpression::from_token_stream(
            tokens,
        )?))
    }

    fn parse_paren(tokens: &mut Tokens) -> Result<Self, CompileError> {
        tokens.get().as_lparen()?;
        let expression = Self::parse(tokens)?;
        tokens.get().as_rparen()?;

        Ok(expression)
    }

    fn parse_primary(tokens: &mut Tokens) -> Result<Self, CompileError> {
        match tokens.peek() {
            Token::Identifier(ident) => Self::parse_ident(tokens),
            Token::UIntLiteral(_) | Token::IntLiteral(_) => Self::parse_constant(tokens),
            Token::LParen => Self::parse_paren(tokens),
            token => Err(CompileError::new(format!(
                "Expected expressions, found {:?}",
                token
            ))),
        }
    }

    fn parse_bin_op_rhs(
        tokens: &mut Tokens,
        precedence: i32,
        mut lhs: Expression,
    ) -> Result<Self, CompileError> {
        loop {
            if *tokens.peek() == Token::Semicolon {
                return Ok(lhs);
            }

            let token_precedance = get_token_precedence(tokens.peek());
            if token_precedance < precedence {
                return Ok(lhs);
            }

            let operator = Operator::from_token_stream(tokens)?;
            let mut rhs = Self::parse_primary(tokens)?;

            let next_precedance = get_token_precedence(tokens.peek());
            if token_precedance < next_precedance {
                rhs = Self::parse_bin_op_rhs(tokens, token_precedance + 1, rhs)?;
            }

            lhs = Expression::Binary(BinaryExpression {
                operator,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            })
        }
    }

    fn parse(tokens: &mut Tokens) -> Result<Self, CompileError> {
        let lhs = Self::parse_primary(tokens)?;
        Self::parse_bin_op_rhs(tokens, 0, lhs)
    }
}

impl FromTokenStream for Expression {
    fn from_token_stream(tokens: &mut Tokens) -> Result<Self, CompileError>
    where
        Self: Sized,
    {
        Self::parse(tokens)
    }
}

lazy_static! {
    static ref BINOP_PRECEDENCE: HashMap<Operator, i32> = {
        let mut map = HashMap::new();

        map.insert(Operator::Less, 10);
        map.insert(Operator::Add, 20);
        map.insert(Operator::Sub, 30);
        map.insert(Operator::Mul, 40);

        map
    };
}

fn get_token_precedence(token: &Token) -> i32 {
    let Ok(operator) = Operator::from_token(token) else {
        return -1;
    };

    let token_precedence = BINOP_PRECEDENCE[&operator];
    if token_precedence <= 0 {
        -1
    } else {
        token_precedence
    }
}
