use super::*;
use std::convert::TryInto;

pub struct BinaryLiteral(pub u8);

impl Instruction for BinaryLiteral {
    fn byte_len(&self) -> u16 { 1 }
    fn compile(&self, _: &HashMap::<String, u16>) -> Vec<u8> {
        vec![self.0]
    }
}

pub struct StringLiteral(pub String);
impl Instruction for StringLiteral {
    fn byte_len(&self) -> u16 {
        self.0.as_bytes().len().try_into().expect("String literal too large")
    }
    fn compile(&self, _: &HashMap::<String, u16>) -> Vec<u8> {
        self.0.as_bytes().to_vec()
    }
}

pub struct ReferenceLiteral(pub String);
impl ReferenceLiteral {
    pub fn get_location(&self, labels: &HashMap::<String, u16>) -> u16 {
        match labels.get(&self.0) {
            Some(location) => *location,
            None => panic!("Invalid reference: {}", self.0),
        }
    }
}
impl Instruction for ReferenceLiteral {
    fn byte_len(&self) -> u16 { 2 }

    fn compile(&self, labels: &HashMap::<String, u16>) -> Vec<u8> {
        self.get_location(labels).to_le_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reflects_binary_literal() {
        let x = BinaryLiteral(255);
        assert_eq!(x.byte_len(), 1);
        assert_eq!(x.compile(&HashMap::<String, u16>::new()), vec![255]);
    }

    #[test]
    fn reflects_string_literal() {
        let x = StringLiteral(String::from("FooBar"));
        assert_eq!(x.byte_len(), 6);
        assert_eq!(x.compile(&HashMap::<String, u16>::new()), vec!['F' as u8, 'o' as u8, 'o' as u8, 'B' as u8, 'a' as u8, 'r' as u8]);
    }

    #[test]
    fn reflects_reference_literal() {
        let x = ReferenceLiteral(String::from("foo_bar"));
        let mut labels = HashMap::<String, u16>::new();
        labels.insert(String::from("foo_bar"), 0xF232);
        assert_eq!(x.byte_len(), 2);
        assert_eq!(x.compile(&labels), vec![0b00110010, 0b11110010]);
    }
}
