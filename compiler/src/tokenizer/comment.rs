use super::*;

pub fn parse(char_iter: &mut CharIterator) -> Result<Token, TokenizationError> {
    let mut comment = String::new();

    // Skip the starting ';' char
    char_iter.next();

    while let Some(c) = char_iter.next() {
        if c == '\n' {
            break;
        }
        
        comment.push(c)
    }
    
    return Ok(Token::Comment(comment));
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
            Ok(Token::Comment(String::from("hello world")))
        );
    }

    #[test]
    fn extracts_comment_with_newline() {
        let code = ";hello world\ntest";
        let mut iter = itertools::multipeek(code.chars());
        assert_eq!(
            parse(&mut iter),
            Ok(Token::Comment(String::from("hello world")))
        );
    }
}
