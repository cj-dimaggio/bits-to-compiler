use super::*;

pub fn is_alphabetic(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

pub fn is_alphanumeric(c: char) -> bool {
    c.is_numeric() || is_alphabetic(c)
}

fn get_word(char_iter: &mut CharIterator) -> String {
    let mut word = String::new();

    while let Some(&c) = char_iter.peek() {
        match c {
            c if is_alphanumeric(c) => word.push(c),
            _ => break
        }
    }

    char_iter.reset_peek();
    word
}

fn parse_number(char_iter: &mut CharIterator) -> Result<Token, TokenizationError> {
    let mut number = String::new();

    while let Some(&c) = char_iter.peek() {
        match c {
            c if c.is_numeric() => {
                number.push(c);
                char_iter.next();
            },
            c if c.is_whitespace() => break,
            _ => {
                return Err(TokenizationError::UnexpectedCharacter);
            }
        }
    }
    let number = number.parse::<i16>().expect("parse_number did not correctly parse a number");
    Ok(Token::Number(number))
}

fn parse_identifier(char_iter: &mut CharIterator) -> Result<Token, TokenizationError> {
    let mut identifier = String::new();

    while let Some(&c) = char_iter.peek() {
        match c {
            c if is_alphanumeric(c) => {
                identifier.push(c);
                char_iter.next();
            },
            c if c.is_whitespace() => break,
            _ => {
                return Err(TokenizationError::UnexpectedCharacter);
            }
        }
    }
    Ok(Token::Reference(identifier))
}

pub fn parse(char_iter: &mut CharIterator) -> Result<Token, TokenizationError> {
    let word = get_word(char_iter);
    let first_char = word.chars().next().expect("alphanumeric::parse called at an invalid cursor position");

    match word {
        word if word.starts_with("0b") => binary_byte::parse(char_iter),
        _ if first_char.is_numeric() => parse_number(char_iter),
        _ if is_alphabetic(first_char) => parse_identifier(char_iter),
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
            parse(&mut itertools::multipeek(code.chars())),
            Ok(Token::Binary(0b11001100))
        );
    }

    #[test]
    fn extracts_number() {
        let code = "1203";
        assert_eq!(
            parse(&mut itertools::multipeek(code.chars())),
            Ok(Token::Number(1203))
        );
    }

    #[test]
    fn extracts_identifier() {
        let code = "foo_bar2";
        assert_eq!(
            parse(&mut itertools::multipeek(code.chars())),
            Ok(Token::Reference(String::from("foo_bar2")))
        );
    }

    #[test]
    fn detects_invalid_number() {
        let code = "120R3";
        assert_eq!(
            parse(&mut itertools::multipeek(code.chars())),
            Err(TokenizationError::UnexpectedCharacter)
        );
    }

    #[test]
    fn detects_invalid_identifier() {
        let code = "foo_&bar";
        assert_eq!(
            parse(&mut itertools::multipeek(code.chars())),
            Err(TokenizationError::UnexpectedCharacter)
        );
    }
}
