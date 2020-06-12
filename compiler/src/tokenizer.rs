#[derive(Debug, PartialEq, Eq)]
pub enum TokenizationError {
    UnexpectedCharacter,
    IncompleteByte
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    BinaryByte([bool; 8]),
    Comment(String),
}

fn parse_binary_byte(char_iter: &mut std::iter::Peekable<std::str::Chars<'_>>) -> Result<Token, TokenizationError> {
    let mut byte = [false, false, false, false, false, false, false, false];
    let mut i = 0;

    while let Some(c) = char_iter.peek() {
        match c {
            '1' | '0' => {
                byte[i] = *c == '1';

                i += 1;
                char_iter.next();

                if i == 8 {
                    break;
                }
            },
            _ => break
        }
    }

    if i < 8 {
        return Err(TokenizationError::IncompleteByte);
    }

    return Ok(Token::BinaryByte(byte));
}

fn parse_comment(char_iter: &mut std::iter::Peekable<std::str::Chars<'_>>) -> Result<Token, TokenizationError> {
    let mut comment = String::new();

    while let Some(c) = char_iter.next() {
        if c == '\n' {
            break;
        }
        
        comment.push(c)
    }
    
    return Ok(Token::Comment(comment));
}

pub fn tokenize(contents: String) -> Result<Vec<Token>, TokenizationError> {
    let mut tokens = Vec::<Token>::new();
    let mut char_iter = contents.chars().peekable();

    while let Some(c) = char_iter.peek() {
        match c {
            '1' | '0' => tokens.push(parse_binary_byte(&mut char_iter)?),
            ';' => {
                // Skip the ;
                char_iter.next();
                tokens.push(parse_comment(&mut char_iter)?);
            },
            c if c.is_whitespace() => {
                char_iter.next();
                continue;
            },
            _ => return Err(TokenizationError::UnexpectedCharacter)
        }
    }

    return Ok(tokens);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_one_binary_byte() {
        let code = "11001100";
        assert_eq!(
            parse_binary_byte(&mut code.chars().peekable()),
            Ok(Token::BinaryByte([true, true, false, false, true, true, false, false]))
        );
    }

    #[test]
    fn extracts_one_binary_byte_from_longer() {
        let code = "1111111100000000";
        let mut iter = code.chars().peekable();
        assert_eq!(
            parse_binary_byte(&mut iter),
            Ok(Token::BinaryByte([true, true, true, true, true, true, true, true]))
        );

        // Make sure that the iterator advanced correctly
        assert_eq!(iter.next(), Some('0'));
    }

    #[test]
    fn extracts_one_binary_byte_with_whitespace() {
        let code = "11110000 ";
        let mut iter = code.chars().peekable();
        assert_eq!(
            parse_binary_byte(&mut iter),
            Ok(Token::BinaryByte([true, true, true, true, false, false, false, false]))
        );

        // Make sure that the iterator advanced correctly
        assert_eq!(iter.next(), Some(' '));
    }

    #[test]
    fn errors_on_incomplete_binary() {
        let code = "1010";
        let mut iter = code.chars().peekable();
        assert_eq!(
            parse_binary_byte(&mut iter),
            Err(TokenizationError::IncompleteByte)
        );
    }

    #[test]
    fn errors_on_interrupted_binary() {
        let code = "1010;1010";
        let mut iter = code.chars().peekable();
        assert_eq!(
            parse_binary_byte(&mut iter),
            Err(TokenizationError::IncompleteByte)
        );
    }

    #[test]
    fn extracts_comment() {
        let code = "hello world";
        let mut iter = code.chars().peekable();
        assert_eq!(
            parse_comment(&mut iter),
            Ok(Token::Comment(String::from("hello world")))
        );
    }

    #[test]
    fn extracts_comment_with_newline() {
        let code = "hello world\ntest";
        let mut iter = code.chars().peekable();
        assert_eq!(
            parse_comment(&mut iter),
            Ok(Token::Comment(String::from("hello world")))
        );
    }

    #[test]
    fn tokenizes_codeblock() {
        let code = "
            11110000 ; Example of a comment
            00000001 10001000
            ; On its own
            11111111 ; Can have 1s and 0s in comment
        ";
        assert_eq!(
            tokenize(String::from(code)),
            Ok(vec![
                Token::BinaryByte([true, true, true, true, false, false, false, false]),
                Token::Comment(String::from(" Example of a comment")),
                Token::BinaryByte([false, false, false, false, false, false, false, true]),
                Token::BinaryByte([true, false, false, false, true, false, false, false]),
                Token::Comment(String::from(" On its own")),
                Token::BinaryByte([true, true, true, true, true, true, true, true]),
                Token::Comment(String::from(" Can have 1s and 0s in comment")),
            ])
        );
    }

    #[test]
    fn tokenization_error() {
        let code = "
            =11110000 ; Example of a comment
        ";
        assert_eq!(
            tokenize(String::from(code)),
            Err(TokenizationError::UnexpectedCharacter)
        );

        let code = "
            1111; Example of a comment
        ";
        assert_eq!(
            tokenize(String::from(code)),
            Err(TokenizationError::IncompleteByte)
        );
    }
}