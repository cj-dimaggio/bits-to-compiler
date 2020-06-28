use super::*;
use super::super::tokenizer::Token;

#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    Assignment {
        identifier: String,
        value: expression::Expression,
    },
    While {
        condition: expression::Expression,
        statements: Vec::<Statement>
    },
    FunctionCall {
        identifier: String,
        params: Vec::<expression::Expression>
    }
}

pub fn parse(token_iter: &mut TokenIterator) -> Result<Option<Statement>, SyntaxError> {
    match token_iter.peek() {
        Some(Token::Let) => {
            token_iter.next();
            let identifier = validate_syntax!(token_iter.next(), Some(Token::Identifier(x)) => x)?;
            validate_syntax!(token_iter.next(), Some(Token::Equals))?;
            let value = expression::parse(token_iter)?;
            validate_syntax!(token_iter.next(), Some(Token::Semicolon))?;
            Ok(Some(Statement::Assignment {
                identifier: identifier.clone(),
                value,
            }))
        },
        Some(Token::While) => {
            token_iter.next();
            validate_syntax!(token_iter.next(), Some(Token::OpenParen))?;
            let condition = expression::parse(token_iter)?;
            validate_syntax!(token_iter.next(), Some(Token::CloseParen))?;
            validate_syntax!(token_iter.next(), Some(Token::OpenBrace))?;
            let mut statements = vec![];
            while let Some(statement) = parse(token_iter)? {
                statements.push(statement);
            }
            validate_syntax!(token_iter.next(), Some(Token::CloseBrace))?;
            Ok(Some(Statement::While {
                condition,
                statements
            }))
        },
        Some(Token::Identifier(value)) => {
            token_iter.next();
            validate_syntax!(token_iter.next(), Some(Token::OpenParen))?;
            let mut params = vec![];
            loop {
                match token_iter.peek() {
                    Some(Token::CloseParen) => {
                        token_iter.next();
                        break;
                    }
                    _ => {
                        let param = expression::parse(token_iter)?;
                        params.push(param);
                    }
                }
            }
            validate_syntax!(token_iter.next(), Some(Token::Semicolon))?;
            Ok(Some(Statement::FunctionCall {
                identifier: value.clone(),
                params,
            }))

        }
        _ => Ok(None)
    }
}
