use super::*;
use super::super::tokenizer::Token;

pub struct OrInstruction {
    // Only support 8 bit registers for now
    operand_1: String,
    operand_2: String,
}

impl OrInstruction {
    pub fn new(tokens: &Vec<Token>) -> Result<OrInstruction, SyntaxError> {
        validate_syntax!(tokens.get(0), Some(Token::Or))?;
        let first = validate_syntax!(tokens.get(1), Some(x) => x)?;
        let operand_1 = match first {
            Token::Register8(reg) => reg,
            _ => return Err(SyntaxError::InvalidParam)
        };
        
        let second = validate_syntax!(tokens.get(2), Some(x) => x)?;
        let operand_2 = match second {
            Token::Register8(reg) => reg,
            _ => return Err(SyntaxError::InvalidParam)
        };

        Ok(OrInstruction {
            operand_1: operand_1.clone(),
            operand_2: operand_2.clone(),
        })
    }
}

impl Instruction for OrInstruction {
    fn byte_len(&self) -> u16 { 2 }

    fn compile(&self, _labels: &HashMap::<String, u16>) -> Vec<u8> {

        // We only handle two register fields for now
        let mod_ = 0b11000000;
        let reg = utils::register_value(&self.operand_1) << 3;
        let r_m = utils::register_value(&self.operand_2);

        vec![0x08, mod_ | reg | r_m]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn or_a_register_to_another() {
        let x = OrInstruction::new(&vec![Token::Or, Token::Register8("ah".to_string()), Token::Register8("cl".to_string())]).unwrap();
        assert_eq!(x.byte_len(), 2);
        assert_eq!(x.compile(&HashMap::<String, u16>::new()), vec![0x08, 0b11_100_001]);
    }
}

