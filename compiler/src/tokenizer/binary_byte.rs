use super::*;

pub fn parse(char_iter: &mut CharIterator) -> Result<Token, TokenizationError> {
    let mut byte = 0;
    let mut i = 0;

    // Skip the 0b prefix
    assert_eq!(char_iter.next(), Some('0'));
    assert_eq!(char_iter.next(), Some('b'));

    while let Some(c) = char_iter.peek() {
        match c {
            '1' | '0' => {
                if *c == '1' {
                    let bit: u8 = 1 << (7 - i);
                    byte |= bit;
                }

                i += 1;
                char_iter.next();
            },
            _ => break
        }
    }

    if i != 8 {
        return Err(TokenizationError::MalformedByte);
    }

    Ok(Token::Binary(byte))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_one_binary_byte() {
        let code = "0b11001100";
        assert_eq!(
            parse(&mut itertools::multipeek(code.chars())),
            Ok(Token::Binary(0b11001100))
        );
    }

    #[test]
    fn extracts_one_binary_byte_from_longer() {
        let code = "0b1111111100000000";
        let mut iter = itertools::multipeek(code.chars());
        assert_eq!(
            parse(&mut iter),
            Err(TokenizationError::MalformedByte)
        );
    }

    #[test]
    fn extracts_one_binary_byte_with_whitespace() {
        let code = "0b11110000 ";
        let mut iter = itertools::multipeek(code.chars());
        assert_eq!(
            parse(&mut iter),
            Ok(Token::Binary(0b11110000))
        );

        // Make sure that the iterator advanced correctly
        assert_eq!(iter.next(), Some(' '));
    }

    #[test]
    fn errors_on_incomplete_binary() {
        let code = "0b1010";
        let mut iter = itertools::multipeek(code.chars());
        assert_eq!(
            parse(&mut iter),
            Err(TokenizationError::MalformedByte)
        );
    }

    #[test]
    fn errors_on_interrupted_binary() {
        let code = "0b1010;1010";
        let mut iter = itertools::multipeek(code.chars());
        assert_eq!(
            parse(&mut iter),
            Err(TokenizationError::MalformedByte)
        );
    }
}
