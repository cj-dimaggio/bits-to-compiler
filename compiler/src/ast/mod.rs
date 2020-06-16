use super::tokenizer::Token;

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    BinaryLiteral(u8),
    StringLiteral(String),
    NumberLiteral(i64),
    Arithmetic{
        first_operand: Box<Expression>,
        second_operand: Box<Expression>,
        operator: char,
    },
}

pub fn parse(tokens: Vec<Token>) -> Vec<Expression> {
    let mut ast = Vec::<Expression>::new();
    let mut cursor = 0;

    while cursor < tokens.len() {
    }

    ast
}
