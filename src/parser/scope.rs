use crate::lexer::{Token, Tokens};
use crate::parser::statement::Statement;
use crate::parser::{CompileError, Element, FromTokenStream};

#[derive(Debug)]
pub struct Scope {
    pub elements: Vec<Statement>,
}

impl FromTokenStream for Scope {
    fn from_token_stream(tokens: &mut Tokens) -> Result<Self, CompileError>
    where
        Self: Sized,
    {
        tokens.get().as_lbrace()?;

        let mut elements = Vec::new();

        while tokens.has_more() && *tokens.peek() != Token::RBrace {
            //We parse assignment
            elements.push(Statement::from_token_stream(tokens)?);
        }

        while *tokens.peek() != Token::RBrace {
            tokens.add_pos(1);
        }

        tokens.get().as_rbrace()?;

        Ok(Self { elements })
    }
}
