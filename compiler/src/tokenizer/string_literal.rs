use super::*;

pub fn parse(char_iter: &mut CharIterator) -> Result<Token, TokenizationError> {
    let mut literal = String::new();

    // Skip the opening quote
    char_iter.next();

    loop {
        match char_iter.next() {
            Some('"') => break,
            Some(c) => {
                literal.push(c);
            },
            None => return Err(TokenizationError::UnterminatedStringLiteral)
        }
    }

    return Ok(Token::StringLiteral(literal));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_string_literal() {
        let code = r#""FooBar""#;
        let mut iter = itertools::multipeek(code.chars());
        assert_eq!(
            parse(&mut iter),
            Ok(Token::StringLiteral(String::from("FooBar")))
        );
    }

    #[test]
    fn catches_unclosed_string_literal() {
        let code = r#""FooBar"#;
        let mut iter = itertools::multipeek(code.chars());
        assert_eq!(
            parse(&mut iter),
            Err(TokenizationError::UnterminatedStringLiteral)
        );
    }
}
