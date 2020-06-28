use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Function {
    identifier: String,
    statements: Vec::<statement::Statement>
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

