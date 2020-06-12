use super::*;

pub fn parse(char_iter: &mut CharIterator) -> Result<Token, TokenizationError> {
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_one_binary_byte() {
        let code = "11001100";
        assert_eq!(
            parse(&mut code.chars().peekable()),
            Ok(Token::BinaryByte([true, true, false, false, true, true, false, false]))
        );
    }

    #[test]
    fn extracts_one_binary_byte_from_longer() {
        let code = "1111111100000000";
        let mut iter = code.chars().peekable();
        assert_eq!(
            parse(&mut iter),
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
            parse(&mut iter),
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
            parse(&mut iter),
            Err(TokenizationError::IncompleteByte)
        );
    }

    #[test]
    fn errors_on_interrupted_binary() {
        let code = "1010;1010";
        let mut iter = code.chars().peekable();
        assert_eq!(
            parse(&mut iter),
            Err(TokenizationError::IncompleteByte)
        );
    }
}
