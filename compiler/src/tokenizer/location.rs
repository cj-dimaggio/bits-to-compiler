use super::*;

pub fn parse(char_iter: &mut CharIterator, current_location: u16, starting_location: u16) -> Result<Token, TokenizationError> {
    // Skip the starting '$' char
    assert_eq!(char_iter.next(), Some('$'));

    match char_iter.peek() {
        Some('$') => {
            // We have a $$
            char_iter.next();
            Ok(Token::Number(i32::from(starting_location)))
        },
        Some(c) if c.is_whitespace() => {
            // We only have a $
            Ok(Token::Number(i32::from(current_location)))
        },
        _ => Err(TokenizationError::UnexpectedCharacter)
    }
}
    


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_starting_location() {
        let code = "$$ + 1";
        assert_eq!(
            parse(&mut code.chars().peekable(), 10, 5),
            Ok(Token::Number(5))
        );
    }

    #[test]
    fn extracts_current_location() {
        let code = "$ + 1";
        assert_eq!(
            parse(&mut code.chars().peekable(), 10, 5),
            Ok(Token::Number(10))
        );
    }

    #[test]
    fn errors_on_invalid() {
        let code = "$&";
        assert_eq!(
            parse(&mut code.chars().peekable(), 10, 5),
            Err(TokenizationError::UnexpectedCharacter)
        );
    }
}
