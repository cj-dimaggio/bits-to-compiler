use super::*;
use super::super::tokenizer::Token;

pub struct JzInstruction {
    position: u16,
    jump_to: String,
}

// Jz and Jmp are pretty much the same thing but with different opcodes. Can we abstract these two?

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
        // We should really be checking that the diff is not greater than a byte
        let diff = to_position.wrapping_sub(self.position + self.byte_len()) as u8; // Remember that counting starts _after_ the jump command 
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
    use super::*;

    #[test]
    fn jumps_to_label_if_zero() {
        let mut labels = HashMap::<String, u16>::new();
        labels.insert("foobar".to_string(), 0x0018);
        let x = JzInstruction::new(&vec![Token::Jz(0x0010), Token::Reference("foobar".to_string())]).unwrap();
        assert_eq!(x.byte_len(), 2);
        assert_eq!(x.compile(&labels), vec![0x74, 6]); // 8 - 2 (counting stats _after_ the jz command)
    }

    #[test]
    fn jumps_to_label_even_if_negative() {
        let mut labels = HashMap::<String, u16>::new();
        labels.insert("foobar".to_string(), 0x0010);
        let x = JmpInstruction::new(&vec![Token::Jmp(0x0011), Token::Reference("foobar".to_string())]).unwrap();
        assert_eq!(x.byte_len(), 2);
        assert_eq!(x.compile(&labels), vec![0xEB, 253]);
    }
}

