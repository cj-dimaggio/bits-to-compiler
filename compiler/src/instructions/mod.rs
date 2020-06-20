use super::tokenizer::Token;

mod literals;

#[derive(Debug, PartialEq, Eq)]
pub enum SyntaxError {
    UnsupportedStartingToken,
    InvalidParam,
    UnexpectedToken,
}

pub trait Instruction {
    fn byte_len(&self) -> u16;
    fn compile(&self) -> Vec<u8>;
}

pub fn parse(tokens: Vec<Token>) -> Result<Box<dyn Instruction>, SyntaxError> {
    debug_assert_ne!(tokens.len(), 0);
    match &tokens[0] {
        Token::Binary(byte) => Ok(Box::new(literals::BinaryLiteral(*byte))),
        Token::QuotedString(data) => Ok(Box::new(literals::StringLiteral(data.clone()))),
        _ => Err(SyntaxError::UnsupportedStartingToken)
    }


}
