use super::*;

pub fn parse(word: &mut String) -> Result<Token, TokenizationError> {
    let mut byte = 0;
    let mut i = 0;

    // Skip the 0b prefix
    assert_eq!(word.remove(0), '0');
    assert_eq!(word.remove(0), 'b');

    for c in word.chars() {
        match c {
            '1' | '0' => {
                if c == '1' {
                    let bit: u8 = 1 << (7 - i);
                    byte |= bit;
                }

                i += 1;
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
            parse(&mut String::from(code)),
            Ok(Token::Binary(0b11001100))
        );
    }

    #[test]
    fn extracts_one_binary_byte_from_longer() {
        let code = "0b1111111100000000";
        assert_eq!(
            parse(&mut String::from(code)),
            Err(TokenizationError::MalformedByte)
        );
    }

    #[test]
    fn extracts_one_binary_byte_with_whitespace() {
        let code = "0b11110000 ";
        assert_eq!(
            parse(&mut String::from(code)),
            Ok(Token::Binary(0b11110000))
        );
    }

    #[test]
    fn errors_on_incomplete_binary() {
        let code = "0b1010";
        assert_eq!(
            parse(&mut String::from(code)),
            Err(TokenizationError::MalformedByte)
        );
    }

    #[test]
    fn errors_on_interrupted_binary() {
        let code = "0b1010;1010";
        assert_eq!(
            parse(&mut String::from(code)),
            Err(TokenizationError::MalformedByte)
        );
    }
}
