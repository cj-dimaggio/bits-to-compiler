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
        param: Option::<expression::Expression>
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
            let mut param = None;

            match token_iter.peek() {
                Some(Token::CloseParen) => {
                    token_iter.next();
                }
                _ => {
                    let value = expression::parse(token_iter)?;
                    param = Some(value);
                    validate_syntax!(token_iter.next(), Some(Token::CloseParen))?;
                }
            }

            validate_syntax!(token_iter.next(), Some(Token::Semicolon))?;
            Ok(Some(Statement::FunctionCall {
                identifier: value.clone(),
                param,
            }))

        }
        _ => Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use expression::Expression;

    #[test]
    fn assignment_statement() {
        assert_eq!(
            parse(&mut vec![
                Token::Let,
                Token::Identifier("foobar".to_string()),
                Token::Equals,
                Token::Number(5),
                Token::Semicolon,
            ].iter().peekable()),
            Ok(Some(Statement::Assignment{
                identifier: "foobar".to_string(),
                value: Expression::NumberLiteral(5),
            }))
        );
    }

    #[test]
    fn function_statement() {
        assert_eq!(
            parse(&mut vec![
                Token::Identifier("foobar".to_string()),
                Token::OpenParen,
                Token::CloseParen,
                Token::Semicolon,
            ].iter().peekable()),
            Ok(Some(Statement::FunctionCall{
                identifier: "foobar".to_string(),
                param: None,
            }))
        );
    }

    #[test]
    fn while_statement() {
        assert_eq!(
            parse(&mut vec![
                Token::While,
                Token::OpenParen,
                Token::Number(1),
                Token::DoesNotEqual,
                Token::Number(2),
                Token::CloseParen,
                Token::OpenBrace,

                Token::Let,
                Token::Identifier("foo".to_string()),
                Token::Equals,
                Token::Number(5),
                Token::Semicolon,

                Token::Identifier("bar".to_string()),
                Token::OpenParen,
                Token::CloseParen,
                Token::Semicolon,

                Token::CloseBrace,
            ].iter().peekable()),
            Ok(Some(Statement::While{
                condition: Expression::NotComparison {
                    left: Box::new(Expression::NumberLiteral(1)),
                    right: Box::new(Expression::NumberLiteral(2)),
                },
                statements: vec![
                    Statement::Assignment {
                        identifier: "foo".to_string(),
                        value: Expression::NumberLiteral(5),
                    },
                    Statement::FunctionCall {
                        identifier: "bar".to_string(),
                        param: None,
                    }
                ],
            }))
        );
    }
}
