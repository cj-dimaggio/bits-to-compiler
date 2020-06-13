use super::*;

pub fn parse(char_iter: &mut CharIterator) -> Result<Token, TokenizationError> {
    let mut comment = String::new();

    // Skip the starting ';' char
    assert_eq!(char_iter.next(), Some(';'));

    while let Some(c) = char_iter.next() {
        // We don't want to swallow newlines
        if c == '\n' {
            break;
        }
        
        comment.push(c)
    }
    
    Ok(Token::Newline)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_comment() {
        let code = ";hello world";
        let mut iter = itertools::multipeek(code.chars());
        assert_eq!(
            parse(&mut iter),
            Ok(Token::Newline)
        );
    }

    #[test]
    fn extracts_comment_with_newline() {
        let code = ";hello world\ntest";
        let mut iter = itertools::multipeek(code.chars());
        assert_eq!(
            parse(&mut iter),
            Ok(Token::Newline)
        );
    }
}
