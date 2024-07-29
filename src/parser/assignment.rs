use crate::lexer::{Token, Tokens};
use crate::parser::expression::Expression;
use crate::parser::{CompileError, FromTokenStream, LiteralType};

#[derive(Debug)]
pub struct AssignmentStatement {
    pub mutable: bool,
    pub ident: String,
    pub rhs: Expression,
}

impl FromTokenStream for AssignmentStatement {
    fn from_token_stream(tokens: &mut Tokens) -> Result<Self, CompileError>
    where
        Self: Sized,
    {
        let mutable = *tokens.get() == Token::Var;
        let ident = tokens.get().as_ident()?.to_owned();
        tokens.get().as_assign()?;
        let rhs = Expression::from_token_stream(tokens)?;
        tokens.get().as_semicolon()?;

        Ok(Self {
            mutable,
            ident,
            rhs,
        })
    }
}
