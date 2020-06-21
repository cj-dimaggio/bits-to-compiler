use super::*;
use std::convert::TryInto;

pub struct BinaryLiteral(pub u8);

impl Instruction for BinaryLiteral {
    fn byte_len(&self) -> u16 { 1 }
    fn compile(&self) -> Vec<u8> {
        vec![self.0]
    }
}

pub struct StringLiteral(pub String);
impl Instruction for StringLiteral {
    fn byte_len(&self) -> u16 {
        self.0.as_bytes().len().try_into().expect("String literal too large")
    }
    fn compile(&self) -> Vec<u8> {
        self.0.as_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reflects_binary_literal() {
        let x = BinaryLiteral(255);
        assert_eq!(x.byte_len(), 1);
        assert_eq!(x.compile(), vec![255]);
    }

    #[test]
    fn reflects_string_literal() {
        let x = StringLiteral(String::from("FooBar"));
        assert_eq!(x.byte_len(), 6);
        assert_eq!(x.compile(), vec!['F' as u8, 'o' as u8, 'o' as u8, 'B' as u8, 'a' as u8, 'r' as u8]);
    }
}
