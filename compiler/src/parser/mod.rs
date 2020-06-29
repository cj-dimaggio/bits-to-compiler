use super::tokenizer::Token;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum SyntaxError {
    UnexpectedToken,
}

type TokenIterator<'a> = std::iter::Peekable<std::slice::Iter<'a, Token>>;

#[macro_use]
mod validate;

pub mod expression;
pub mod function;
pub mod statement;

#[derive(Debug, PartialEq, Eq)]
pub struct Program {
    pub functions: Vec<function::Function>,
    pub statements: Vec<statement::Statement>,
}

pub fn parse(tokens: Vec<Token>) -> Result<Program, SyntaxError> {
    let mut token_iter = tokens.iter().peekable();

    let mut functions = vec![];
    let mut statements = vec![];

    while let Some(token) = token_iter.peek() {
        if let Token::Function = token {
            let function = function::parse(&mut token_iter)?;
            functions.push(function);
        } else if let Some(statement) = statement::parse(&mut token_iter)? {
            statements.push(statement)
        } else {
            return Err(SyntaxError::UnexpectedToken);
        }
    }

    Ok(Program {
        functions,
        statements,
    })
}

#[cfg(test)]
mod tests {
    use super::super::tokenizer;
    use super::*;
    use expression::Expression;
    use function::Function;
    use statement::Statement;

    #[test]
    fn parse_codeblock() {
        let code = r#"
        let hello_world = "Hello, World!";

        fn main() {
            let i = 0;
        }        
        "#
        .to_string();

        assert_eq!(
            parse(tokenizer::tokenize(code).unwrap()),
            Ok(Program {
                statements: vec![Statement::Assignment {
                    identifier: "hello_world".to_string(),
                    value: Expression::StringLiteral("Hello, World!".to_string()),
                }],
                functions: vec![Function {
                    identifier: "main".to_string(),
                    statements: vec![Statement::Assignment {
                        identifier: "i".to_string(),
                        value: Expression::NumberLiteral(0),
                    },],
                }],
            })
        );
    }
}
