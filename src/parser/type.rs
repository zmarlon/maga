use crate::parser::*;

#[derive(Debug)]
pub struct Type {
    pub name: String,
    pub is_pointer: bool,
}

impl Type {
    pub fn void() -> Self {
        Self {
            name: "()".to_owned(),
            is_pointer: false,
        }
    }
}

impl FromTokenStream for Type {
    fn from_token_stream(tokens: &mut Tokens) -> Result<Self, CompileError>
    where
        Self: Sized,
    {
        let mut is_pointer = false;

        //TODO: we have to parse things like double pointers etc.
        if *tokens.peek() == Token::Mul {
            tokens.add_pos(1);
            is_pointer = true;
        }

        let name = tokens.get().as_ident()?.to_owned();

        Ok(Self { name, is_pointer })
    }
}
