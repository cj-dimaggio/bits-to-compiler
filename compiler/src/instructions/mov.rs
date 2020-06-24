use super::*;
use super::super::tokenizer::Token;

pub struct MovInstruction {
    destination: Token,
    input: Token,
}

// No static hashmaps with current compile-time checker
fn register_value(reg: &String) -> u8 {
    match &reg.to_lowercase()[..] {
        "al" | "ax" => 0,
        "cl" | "cx" => 1,
        "dl" | "dx" => 2,
        "bl" | "bx" => 3,
        "ah" | "sp" => 4,
        "ch" | "bp" => 5,
        "dh" | "si" => 6,
        "bh" | "di" => 7,
        _ => panic!("Invalid eight bit register {}", reg)
    }
}

impl MovInstruction {
    pub fn new(tokens: &Vec<Token>) -> Result<MovInstruction, SyntaxError> {
        validate_syntax!(tokens.get(0), Some(Token::Mov))?;
        let first = validate_syntax!(tokens.get(1), Some(x) => x)?;
        let destination = match first {
            Token::Register8(_) | Token::Register16(_) => first,
            _ => return Err(SyntaxError::InvalidParam)
        };
        
        let second = validate_syntax!(tokens.get(2), Some(x) => x)?;
        let input = match second {
            Token::Binary(_) | Token::Reference(_) => second,
            _ => return Err(SyntaxError::InvalidParam)
        };

        Ok(MovInstruction {
            destination: destination.clone(),
            input: input.clone(),
        })
    }
}

impl Instruction for MovInstruction {
    fn byte_len(&self) -> u16 {
        match self.destination {
            Token::Register8(_) => 2,
            Token::Register16(_) => 3,
            _ => panic!("MovInstruction created with invalid params"),
        }
    }

    fn compile(&self, labels: &HashMap::<String, u16>) -> Vec<u8> {
        let destination = &self.destination;
        let input = &self.input;

        match destination {
            Token::Register8(reg) => {
                let val = validate_syntax!(input, Token::Binary(x) => x).expect("MovInstruction created with invalid params");
                vec![0xB0 + register_value(reg), *val]
            },
            Token::Register16(reg) => {
                let label = validate_syntax!(input, Token::Reference(x) => x).expect("MovInstruction created with invalid params");
                let referance = literals::ReferenceLiteral(label.to_string());
                let mut response = vec![0xB8 + register_value(reg)];
                response.append(&mut referance.compile(labels));
                response
            },
            _ => panic!("MovInstruction created with invalid params"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mov_8_bit_number() {
        let x = MovInstruction::new(&vec![Token::Mov, Token::Register8("ah".to_string()), Token::Binary(0x55)]).unwrap();
        assert_eq!(x.byte_len(), 2);
        assert_eq!(x.compile(&HashMap::<String, u16>::new()), vec![0xB4, 0x55]);
    }

    #[test]
    fn mov_16_bit_reference() {
        let mut labels = HashMap::<String, u16>::new();
        labels.insert("foobar".to_string(), 0x2468);
        let x = MovInstruction::new(&vec![Token::Mov, Token::Register16("dx".to_string()), Token::Reference("foobar".to_string())]).unwrap();
        assert_eq!(x.byte_len(), 3);
        assert_eq!(x.compile(&labels), vec![0xBa, 0x68, 0x24]);
    }
}
