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
