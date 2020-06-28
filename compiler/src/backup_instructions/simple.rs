use super::*;

pub struct CliInstruction;
impl Instruction for CliInstruction {
    fn byte_len(&self) -> u16 { 1 }
    fn compile(&self, _: &HashMap::<String, u16>) -> Vec<u8> {
        vec![0xFA]
    }
}

pub struct HltInstruction;
impl Instruction for HltInstruction {
    fn byte_len(&self) -> u16 { 1 }
    fn compile(&self, _: &HashMap::<String, u16>) -> Vec<u8> {
        vec![0xF4]
    }
}

// Only handles the no-operand scenario
pub struct LodsbInstruction;
impl Instruction for LodsbInstruction {
    fn byte_len(&self) -> u16 { 1 }
    fn compile(&self, _: &HashMap::<String, u16>) -> Vec<u8> {
        vec![0xAC]
    }
}

pub struct InterruptInstruction(u8);

impl InterruptInstruction {
    pub fn new(tokens: &Vec<Token>) -> Result<InterruptInstruction, SyntaxError> {
        validate_syntax!(tokens.get(0), Some(Token::Interrupt))?;
        let interrupt = validate_syntax!(tokens.get(1), Some(Token::Binary(x)) => x)?;
        validate_syntax!(tokens.get(2), None)?;
        Ok(InterruptInstruction(*interrupt))
    }
}

impl Instruction for InterruptInstruction {
    fn byte_len(&self) -> u16 { 2 }
    fn compile(&self, _: &HashMap::<String, u16>) -> Vec<u8> {
        vec![0xCD, self.0]
    }
}
