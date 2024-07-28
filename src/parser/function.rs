use crate::parser::scope::Scope;
use crate::parser::*;

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub return_type: Type,

    pub params: Vec<FunctionParam>,

    pub body: Option<Scope>,
}

impl FromTokenStream for Function {
    fn from_token_stream(tokens: &mut Tokens) -> Result<Self, CompileError> {
        tokens.add_pos(1);

        let mut params = Vec::new();
        let mut return_type = Type::void();

        //Parse ident
        let name = tokens.get().as_ident()?.to_owned();

        //Expect lparen
        tokens.get().as_lparen()?;

        //Expect parameter list
        while *tokens.peek() != Token::RParen {
            params.push(FunctionParam::from_token_stream(tokens)?);
        }

        tokens.get().as_rparen()?;

        //Parse optional return type
        if *tokens.peek() == Token::DoubleColon {
            tokens.add_pos(1);
            return_type = Type::from_token_stream(tokens)?;
        }

        let mut body = None;

        //We check if function has body
        if *tokens.peek() == Token::LBrace {
            body = Some(Scope::from_token_stream(tokens)?);
        }

        Ok(Self {
            name,
            return_type,
            params,
            body,
        })
    }
}

#[derive(Debug)]
pub struct FunctionParam {
    pub name: String,
    pub r#type: Type,
}

impl FromTokenStream for FunctionParam {
    fn from_token_stream(tokens: &mut Tokens) -> Result<Self, CompileError>
    where
        Self: Sized,
    {
        let name = tokens.get().as_ident()?.to_owned();
        tokens.get().as_double_colon()?;

        let mut r#type = Type::from_token_stream(tokens)?;

        if *tokens.try_peek()? == Token::Comma {
            tokens.add_pos(1);
        }

        Ok(Self { name, r#type })
    }
}
