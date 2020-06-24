use super::*;
use super::super::tokenizer::Token;

pub struct JzInstruction {
    position: u16,
    jump_to: String,
}

impl JzInstruction {
    pub fn new(tokens: &Vec<Token>) -> Result<JzInstruction, SyntaxError> {
        let position = validate_syntax!(tokens.get(0), Some(Token::Jz(x)) => *x)?;
        let jump_to = validate_syntax!(tokens.get(1), Some(Token::Reference(x)) => x.to_string())?;
        validate_syntax!(tokens.get(2), None)?;

        Ok(JzInstruction {
            position,
            jump_to,
        })
    }
}

impl Instruction for JzInstruction {
    fn byte_len(&self) -> u16 { 2 }

    fn compile(&self, labels: &HashMap::<String, u16>) -> Vec<u8> {
        let to_position = labels.get(&self.jump_to).unwrap();
        let diff = to_position.wrapping_sub(self.position + self.byte_len()) as u8;
        vec![0x74, diff]
    }
}

pub struct JmpInstruction {
    position: u16,
    jump_to: String,
}

impl JmpInstruction {
    pub fn new(tokens: &Vec<Token>) -> Result<JmpInstruction, SyntaxError> {
        let position = validate_syntax!(tokens.get(0), Some(Token::Jmp(x)) => *x)?;
        let jump_to = validate_syntax!(tokens.get(1), Some(Token::Reference(x)) => x.to_string())?;
        validate_syntax!(tokens.get(2), None)?;

        Ok(JmpInstruction {
            position,
            jump_to,
        })
    }
}

impl Instruction for JmpInstruction {
    fn byte_len(&self) -> u16 { 2 }

    fn compile(&self, labels: &HashMap::<String, u16>) -> Vec<u8> {
        let to_position = labels.get(&self.jump_to).unwrap();
        let diff = to_position.wrapping_sub(self.position + self.byte_len()) as u8;
        vec![0xEB, diff]
    }
}

#[cfg(test)]
mod tests {
}

