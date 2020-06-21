use super::tokenizer::Token;

#[macro_use]
mod validate;

mod literals;
mod times;

#[derive(Debug, PartialEq, Eq)]
pub enum SyntaxError {
    UnsupportedStartingToken,
    InvalidParam,
    NumberCanNotBeNegative,
}

pub trait Instruction {
    fn byte_len(&self) -> u16;
    fn compile(&self) -> Vec<u8>;
}

pub fn extract_instruction(tokens: &Vec<Token>) -> Result<Box<dyn Instruction>, SyntaxError> {
    debug_assert_ne!(tokens.len(), 0);
    match &tokens[0] {
        Token::Binary(byte) => {
            validate_syntax!(tokens.get(1), None)?;
            Ok(Box::new(literals::BinaryLiteral(*byte)))
        },
        Token::QuotedString(data) => {
            validate_syntax!(tokens.get(1), None)?;
            Ok(Box::new(literals::StringLiteral(data.clone())))
        },
        _ => Err(SyntaxError::UnsupportedStartingToken)
    }
}
