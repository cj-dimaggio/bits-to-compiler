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