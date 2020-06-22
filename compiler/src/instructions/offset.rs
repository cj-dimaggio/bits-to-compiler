use super::*;
use literals::ReferenceLiteral;
use super::super::tokenizer::Token;

pub struct OffsetDirective{
    amount: i16,
    reference: ReferenceLiteral,
}

impl OffsetDirective {
    pub fn new(tokens: &Vec<Token>) -> Result<OffsetDirective, SyntaxError> {
        validate_syntax!(tokens.get(0), Some(Token::Offset))?;
        let amount = validate_syntax!(tokens.get(2), Some(Token::Number(x)) => *x )?;
        validate_syntax!(tokens.get(3), None)?;

        match tokens.get(1) {
            Some(Token::Reference(label)) => Ok(OffsetDirective {
                amount,
                reference: ReferenceLiteral(label.clone()),
            }),
            Some(_) | None => Err(SyntaxError::InvalidParam)
        }
    }
}

impl Instruction for OffsetDirective {
    fn byte_len(&self) -> u16 { 2 }

    fn compile(&self, labels: &HashMap::<String, u16>) -> Vec<u8> {
        let offset = self.reference.get_location(labels).wrapping_add(self.amount as u16);
        offset.to_le_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_positive_offset() {
        let mut labels = HashMap::<String, u16>::new();
        labels.insert(String::from("foo_bar"), 100);
        let x = OffsetDirective::new(&vec![Token::Offset, Token::Reference(String::from("foo_bar")), Token::Number(200)]).unwrap();
        assert_eq!(x.byte_len(), 2);
        let bytes = x.compile(&labels);
        let num = ((bytes[1] as u16) << 8) + bytes[0] as u16;
        assert_eq!(num, 300);
    }

    #[test]
    fn create_negative_offset() {
        let mut labels = HashMap::<String, u16>::new();
        labels.insert(String::from("foo_bar"), 100);
        let x = OffsetDirective::new(&vec![Token::Offset, Token::Reference(String::from("foo_bar")), Token::Number(-25)]).unwrap();
        assert_eq!(x.byte_len(), 2);
        let bytes = x.compile(&labels);
        let num = ((bytes[1] as u16) << 8) + bytes[0] as u16;
        assert_eq!(num, 75);
    }
}
