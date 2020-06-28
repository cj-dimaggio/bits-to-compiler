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

pub fn parse(token_iter: &mut TokenIterator) -> Result<Expression, SyntaxError> {
    let exp = match token_iter.next() {
        Some(Token::Number(num)) => Expression::NumberLiteral(*num),
        Some(Token::QuotedString(value)) => Expression::StringLiteral(value.clone()),
        Some(Token::Identifier(value)) => Expression::Variable(value.clone()),
        _ => return Err(SyntaxError::UnexpectedToken),
    };

    match token_iter.peek() {
        Some(Token::Plus) => {
            token_iter.next();
            let right = parse(token_iter)?;
            Ok(Expression::Addition {
                left: Box::new(exp),
                right: Box::new(right),
            })
        }
        Some(Token::DoesNotEqual) => {
            token_iter.next();
            let right = parse(token_iter)?;
            Ok(Expression::NotComparison {
                left: Box::new(exp),
                right: Box::new(right),
            })
        },
        Some(Token::OpenBracket) => {
            token_iter.next();
            let inner = parse(token_iter)?;
            validate_syntax!(token_iter.next(), Some(Token::CloseBracket))?;
            Ok(Expression::Lookup {
                base: Box::new(exp),
                index: Box::new(inner),
            })
        },
        _ => Ok(exp)
    }
}
