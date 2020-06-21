use super::*;

pub fn parse(char_iter: &mut CharIterator) -> Result<Token, TokenizationError> {
    let mut literal = String::new();

    // Skip the opening quote
    assert_eq!(char_iter.next(), Some('"'));

    loop {
        match char_iter.next() {
            Some('"') => break,
            Some(c) => {
                literal.push(c);
            },
            None => return Err(TokenizationError::UnterminatedStringLiteral)
        }
    }

    Ok(Token::QuotedString(literal))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_string_literal() {
        let code = r#""FooBar""#;
        assert_eq!(
            parse(&mut code.chars().peekable()),
            Ok(Token::QuotedString(String::from("FooBar")))
        );
    }

    #[test]
    fn catches_unclosed_string_literal() {
        let code = r#""FooBar"#;
        assert_eq!(
            parse(&mut code.chars().peekable()),
            Err(TokenizationError::UnterminatedStringLiteral)
        );
    }
}
