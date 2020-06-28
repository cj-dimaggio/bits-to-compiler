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
            c if is_alphanumeric(c) => {
                word.push(c);
                char_iter.next();
            },
            _ => break,
        }
    }

    word
}

fn parse_number(word: String) -> Result<Token, TokenizationError> {
    if let Ok(number) = word.parse::<i16>() {
        Ok(Token::Number(number))
    } else {
        Err(TokenizationError::UnexpectedCharacter)
    }
}

pub fn parse(char_iter: &mut CharIterator) -> Result<Token, TokenizationError> {
    let word = get_word(char_iter);
    let first_char = word.chars().next().expect("alphanumeric::parse called at an invalid cursor position");

    match &word[..] {
        "while" => Ok(Token::While),
        "let" => Ok(Token::Let),
        "int" => Ok(Token::Int),
        "void" => Ok(Token::Void),
        _ if first_char.is_numeric() => parse_number(word),
        _ if is_alphabetic(first_char) => Ok(Token::Identifier(word)),
        _ => Err(TokenizationError::UnexpectedCharacter)
    }
}
