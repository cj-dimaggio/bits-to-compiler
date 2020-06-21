use super::*;
use super::super::tokenizer::Token;

pub struct TimesInstruction{
    amount: u16,
    instruction: Box<dyn Instruction>
}

impl TimesInstruction {
    pub fn new(tokens: &Vec<Token>) -> Result<TimesInstruction, SyntaxError> {
        validate_syntax!(tokens.get(0), Some(Token::Times))?;
        let amount = validate_syntax!(tokens.get(1), Some(Token::Number(x)) => *x )?;
        if amount < 0 {
            return Err(SyntaxError::NumberCanNotBeNegative);
        }
        
        validate_syntax!(tokens.get(2), Some(_))?;
        let instruction = extract_instruction(&tokens[2..].to_vec())?;

        Ok(TimesInstruction {
            amount: amount as u16,
            instruction,
        })
    }
}

impl Instruction for TimesInstruction {
    fn byte_len(&self) -> u16 {
        self.instruction.byte_len() * self.amount
    }

    fn compile(&self) -> Vec<u8> {
        let bytes = self.instruction.compile();
        let mut output = Vec::<u8>::new();

        for _ in 0..self.amount {
            output.append(&mut bytes.clone());
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_multiple_binary_literals() {
        let x = TimesInstruction::new(&vec![Token::Times, Token::Number(5), Token::Binary(255)]).unwrap();
        assert_eq!(x.byte_len(), 5);
        assert_eq!(x.compile(), vec![255, 255, 255, 255, 255]);
    }

    #[test]
    fn create_multiple_string_literals() {
        let x = TimesInstruction::new(&vec![Token::Times, Token::Number(3), Token::QuotedString("Foo".to_string())]).unwrap();
        assert_eq!(x.byte_len(), 9);
        assert_eq!(x.compile(), vec!['F' as u8, 'o' as u8, 'o' as u8, 'F' as u8, 'o' as u8, 'o' as u8, 'F' as u8, 'o' as u8, 'o' as u8]);
    }
}
