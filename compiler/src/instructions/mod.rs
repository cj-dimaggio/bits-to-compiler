use super::tokenizer::Token;
use std::collections::HashMap;

#[macro_use]
mod validate;

mod literals;
mod times;
pub mod directives;
mod simple;
mod mov;
mod utils;
mod or;
mod jump;

#[derive(Debug, PartialEq, Eq)]
pub enum SyntaxError {
    UnsupportedStartingToken,
    InvalidParam,
    NumberCanNotBeNegative,
}

pub trait Instruction {
    fn byte_len(&self) -> u16;
    fn compile(&self, labels: &HashMap::<String, u16>) -> Vec<u8>;
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
        Token::Cli => {
            validate_syntax!(tokens.get(1), None)?;
            Ok(Box::new(simple::CliInstruction))
        },
        Token::Hlt => {
            validate_syntax!(tokens.get(1), None)?;
            Ok(Box::new(simple::HltInstruction))
        },
        Token::Lodsb => {
            validate_syntax!(tokens.get(1), None)?;
            Ok(Box::new(simple::LodsbInstruction))
        },
        Token::Interrupt => Ok(Box::new(simple::InterruptInstruction::new(tokens)?)),
        Token::Times => Ok(Box::new(times::TimesDirective::new(tokens)?)),
        Token::Offset => Ok(Box::new(directives::OffsetDirective::new(tokens)?)),
        Token::Mov => Ok(Box::new(mov::MovInstruction::new(tokens)?)),
        Token::Or => Ok(Box::new(or::OrInstruction::new(tokens)?)),
        Token::Jz(_) => Ok(Box::new(jump::JzInstruction::new(tokens)?)),
        Token::Jmp(_) => Ok(Box::new(jump::JmpInstruction::new(tokens)?)),
        _ => Err(SyntaxError::UnsupportedStartingToken)
    }
}
