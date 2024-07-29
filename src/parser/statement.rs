use crate::lexer::{Token, Tokens};
use crate::parser::assignment::AssignmentStatement;
use crate::parser::expression::Expression;
use crate::parser::{CompileError, FromTokenStream};

#[derive(Debug)]
pub enum Statement {
    Return(ReturnStatement),
    Declaration(AssignmentStatement),
}

#[derive(Debug)]
pub struct ReturnStatement {
    expression: Option<Expression>,
}

impl FromTokenStream for ReturnStatement {
    fn from_token_stream(tokens: &mut Tokens) -> Result<Self, CompileError>
    where
        Self: Sized,
    {
        tokens.get().as_return()?;
        let mut expression = None;

        if *tokens.peek() != Token::Semicolon {
            expression = Some(Expression::from_token_stream(tokens)?);
        }

        tokens.get().as_semicolon()?;
        Ok(Self { expression })
    }
}

impl FromTokenStream for Statement {
    fn from_token_stream(tokens: &mut Tokens) -> Result<Self, CompileError>
    where
        Self: Sized,
    {
        match tokens.peek() {
            Token::Var | Token::Let => Ok(Statement::Declaration(
                AssignmentStatement::from_token_stream(tokens)?,
            )),
            Token::Return => Ok(Statement::Return(ReturnStatement::from_token_stream(
                tokens,
            )?)),
            token => Err(CompileError::new(format!(
                "Expected statement, found {:?}",
                token
            ))),
        }
    }
}
