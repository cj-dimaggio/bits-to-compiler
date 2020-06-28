use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    NumberLiteral(i16),
    StringLiteral(String),
    Variable(String),
    Lookup {
        base: Box::<Expression>,
        index: Box::<Expression>,
    },
    NotComparison {
        left: Box::<Expression>,
        right: Box::<Expression>,
    },
    Addition {
        left: Box::<Expression>,
        right: Box::<Expression>,
    }
}

fn get_value(token: Option<&Token>) -> Result<Expression, SyntaxError> {
    Ok(match token {
        Some(Token::Number(num)) => Expression::NumberLiteral(*num),
        Some(Token::QuotedString(value)) => Expression::StringLiteral(value.clone()),
        Some(Token::Identifier(value)) => Expression::Variable(value.clone()),
        _ => return Err(SyntaxError::UnexpectedToken),
    })
}

pub fn parse(token_iter: &mut TokenIterator) -> Result<Expression, SyntaxError> {
    let mut exp = get_value(token_iter.next())?;

    while let Some(token) = token_iter.peek() {
        match token {
            Token::Plus => {
                token_iter.next();
                let right = parse(token_iter)?;
                exp = Expression::Addition {
                    left: Box::new(exp),
                    right: Box::new(right),
                };
            },
        Token::DoesNotEqual => {
            token_iter.next();
            let right = parse(token_iter)?;
            exp = Expression::NotComparison {
                left: Box::new(exp),
                right: Box::new(right),
            }
        },
        Token::OpenBracket => {
            token_iter.next();
            let inner = parse(token_iter)?;
            validate_syntax!(token_iter.next(), Some(Token::CloseBracket))?;
            exp = Expression::Lookup {
                base: Box::new(exp),
                index: Box::new(inner),
            }
        },
        _ => break,
        }
    }
    
    Ok(exp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_expressions() {
        assert_eq!(
            parse(&mut vec![
                Token::Number(5),
            ].iter().peekable()),
            Ok(Expression::NumberLiteral(5))
        );
        assert_eq!(
            parse(&mut vec![
                Token::QuotedString("Test".to_string()),
            ].iter().peekable()),
            Ok(Expression::StringLiteral("Test".to_string()))
        );
        assert_eq!(
            parse(&mut vec![
                Token::Identifier("foobar".to_string()),
            ].iter().peekable()),
            Ok(Expression::Variable("foobar".to_string()))
        );
    }

    #[test]
    fn lookup_expressions() {
        assert_eq!(
            parse(&mut vec![
                Token::Identifier("foobar".to_string()),
                Token::OpenBracket,
                Token::Number(5),
                Token::CloseBracket,
            ].iter().peekable()),
            Ok(Expression::Lookup{
                base: Box::new(Expression::Variable("foobar".to_string())),
                index: Box::new(Expression::NumberLiteral(5)),
            })
        );
    }

    #[test]
    fn not_comparison_expressions() {
        assert_eq!(
            parse(&mut vec![
                Token::Number(5),
                Token::DoesNotEqual,
                Token::Number(1),
            ].iter().peekable()),
            Ok(Expression::NotComparison{
                left: Box::new(Expression::NumberLiteral(5)),
                right: Box::new(Expression::NumberLiteral(1)),
            })
        );
    }

    #[test]
    fn addition_expressions() {
        assert_eq!(
            parse(&mut vec![
                Token::Number(5),
                Token::Plus,
                Token::Number(1),
            ].iter().peekable()),
            Ok(Expression::Addition{
                left: Box::new(Expression::NumberLiteral(5)),
                right: Box::new(Expression::NumberLiteral(1)),
            })
        );
    }

    #[test]
    fn complex_expressions() {
        // foobar[i + 1] != 5 + 4
        assert_eq!(
            parse(&mut vec![
                Token::Identifier("foobar".to_string()),
                Token::OpenBracket,
                Token::Identifier("i".to_string()),
                Token::Plus,
                Token::Number(1),
                Token::CloseBracket,
                Token::DoesNotEqual,
                Token::Number(5),
                Token::Plus,
                Token::Number(4)
            ].iter().peekable()),
            Ok(Expression::NotComparison{
                left: Box::new(Expression::Lookup{
                    base: Box::new(Expression::Variable("foobar".to_string())),
                    index: Box::new(Expression::Addition {
                        left: Box::new(Expression::Variable("i".to_string())),
                        right: Box::new(Expression::NumberLiteral(1))
                    }),
                }),
                right: Box::new(Expression::Addition{
                    left: Box::new(Expression::NumberLiteral(5)),
                    right: Box::new(Expression::NumberLiteral(4)),
                }),
            })
        );
    }
}

