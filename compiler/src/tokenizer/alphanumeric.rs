use super::*;

pub fn is_alphabetic(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

pub fn is_alphanumeric(c: char) -> bool {
    c.is_numeric() || is_alphabetic(c)
}

fn get_word(char_iter: &mut CharIterator) -> Result<String, TokenizationError> {
    let mut word = String::new();

    while let Some(&c) = char_iter.peek() {
        match c {
            c if is_alphanumeric(c) => {
                word.push(c);
                char_iter.next();
            },
            ':' => {
                word.push(c);
                char_iter.next();
                break
            },
            c if c.is_whitespace() || c == ';' => break,
            _ => {
                return Err(TokenizationError::UnexpectedCharacter);
            }
        }
    }

    Ok(word)
}

fn parse_number(word: String) -> Result<Token, TokenizationError> {
    if let Ok(number) = word.parse::<i16>() {
        Ok(Token::Number(number))
    } else {
        Err(TokenizationError::UnexpectedCharacter)
    }
}

fn parse_hex(word: String) -> Result<Token, TokenizationError> {
    debug_assert_eq!(word.starts_with("0x"), true);
    let without_prefix = word.trim_start_matches("0x");
    if without_prefix.len() == 2 {
        if let Ok(byte) = u8::from_str_radix(without_prefix, 16) {
            return Ok(Token::Binary(byte));
        } else {
            return Err(TokenizationError::InvalidHex);
        }
    } else if without_prefix.len() == 4 {
        if let Ok(number) = i16::from_str_radix(without_prefix, 16) {
            return Ok(Token::Number(number));
        } else {
            return Err(TokenizationError::InvalidHex);
        }
    } else {
        Err(TokenizationError::InvalidHex)
    }
}

pub fn parse(char_iter: &mut CharIterator) -> Result<Token, TokenizationError> {
    let mut word = get_word(char_iter)?;
    let first_char = word.chars().next().expect("alphanumeric::parse called at an invalid cursor position");

    match &word.to_lowercase()[..] {
        "times" => Ok(Token::Times),
        "offset" => Ok(Token::Offset),
        word if word.starts_with("0b") => binary_byte::parse(word.to_string()),
        word if word.starts_with("0x") => parse_hex(word.to_string()),
        _ if first_char.is_numeric() => parse_number(word),
        _ if is_alphabetic(first_char) => {
            if word.ends_with(':') {
                word.remove(word.len() - 1);
                Ok(Token::Label(word))
            } else {
                Ok(Token::Reference(word))
            }
        },
        _ => Err(TokenizationError::UnexpectedCharacter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_binary_byte() {
        let code = "0b11001100";
        assert_eq!(
            parse(&mut code.chars().peekable()),
            Ok(Token::Binary(0b11001100))
        );
    }

    #[test]
    fn extracts_number() {
        let code = "1203";
        assert_eq!(
            parse(&mut code.chars().peekable()),
            Ok(Token::Number(1203))
        );
    }

    #[test]
    fn parse_reference() {
        let code = "foo_bar2";
        assert_eq!(
            parse(&mut code.chars().peekable()),
            Ok(Token::Reference(String::from("foo_bar2")))
        );
    }

    #[test]
    fn parse_label() {
        let code = "foo_bar:";
        assert_eq!(
            parse(&mut code.chars().peekable()),
            Ok(Token::Label(String::from("foo_bar")))
        );
    }

    #[test]
    fn detects_invalid_number() {
        let code = "120R3";
        assert_eq!(
            parse(&mut code.chars().peekable()),
            Err(TokenizationError::UnexpectedCharacter)
        );
    }

    #[test]
    fn detects_invalid_identifier() {
        let code = "foo_&bar";
        assert_eq!(
            parse(&mut code.chars().peekable()),
            Err(TokenizationError::UnexpectedCharacter)
        );
    }

    #[test]
    fn parses_hex_into_a_byte() {
        let code = "0xF1";
        assert_eq!(
            parse(&mut code.chars().peekable()),
            Ok(Token::Binary(0xF1))
        );
    }

    #[test]
    fn parses_hex_into_a_number() {
        let code = "0x11F2";
        assert_eq!(
            parse(&mut code.chars().peekable()),
            Ok(Token::Number(0x11F2))
        );
    }
}
