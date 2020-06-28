use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Function {
    pub identifier: String,
    pub statements: Vec::<statement::Statement>
}

pub fn parse(token_iter: &mut TokenIterator) -> Result<Function, SyntaxError> {
    validate_syntax!(token_iter.next(), Some(Token::Function))?;
    let identifier = validate_syntax!(token_iter.next(), Some(Token::Identifier(x)) => x)?;
    validate_syntax!(token_iter.next(), Some(Token::OpenParen))?;
    validate_syntax!(token_iter.next(), Some(Token::CloseParen))?;
    validate_syntax!(token_iter.next(), Some(Token::OpenBrace))?;

    let mut statements = vec![];
    while let Some(statement) = statement::parse(token_iter)? {
        statements.push(statement);
    }

    validate_syntax!(token_iter.next(), Some(Token::CloseBrace))?;

    Ok(Function {
        identifier: identifier.clone(),
        statements,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use statement::Statement;
    use expression::Expression;

    #[test]
    fn define_function() {
        assert_eq!(
            parse(&mut vec![
                Token::Function,
                Token::Identifier("main".to_string()),
                Token::OpenParen,
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
            Ok(Function {
                identifier: "main".to_string(),
                statements: vec![
                    Statement::Assignment {
                        identifier: "foo".to_string(),
                        value: Expression::NumberLiteral(5),
                    },
                    Statement::FunctionCall {
                        identifier: "bar".to_string(),
                        params: vec![],
                    }
                ],
            })
        );
    }
}
