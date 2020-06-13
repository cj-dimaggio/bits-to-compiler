#[derive(Debug, PartialEq, Eq)]
pub enum TokenizationError {
    UnexpectedCharacter,
    IncompleteByte,
    UnterminatedStringLiteral
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    BinaryByte([bool; 8]),
    StringLiteral(String),
    Number(i64),
    Identifier(String),
    Comment(String),
}

type CharIterator<'a> = itertools::MultiPeek<std::str::Chars<'a>>;

mod binary_byte;
mod string_literal;
mod alphanumeric;
mod comment;

pub fn tokenize(contents: String) -> Result<Vec<Token>, TokenizationError> {
    let mut tokens = Vec::<Token>::new();
    let mut char_iter = itertools::multipeek(contents.chars());

    while let Some(&c) = char_iter.peek() {
        // Make sure our parsers can grab the character we just pulled
        char_iter.reset_peek();

        tokens.push(match c {
            ';' => comment::parse(&mut char_iter)?,
            '"' => string_literal::parse(&mut char_iter)?,
            c if alphanumeric::is_alphanumeric(c) => alphanumeric::parse(&mut char_iter)?,
            c if c.is_whitespace() => {
                char_iter.next();
                continue;
            },
            _ => return Err(TokenizationError::UnexpectedCharacter)
        });

        // Cleanup after any of our parsers
        char_iter.reset_peek();
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizes_codeblock() {
        let code = r#"
            0b11110000 ; Example of a comment
            0b00000001 0b10001000
            ; On its own
            0b11111111 ; Can have 1s and 0s in comment
            "This is a test"
            1234
            Hello_World
        "#;
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
                Token::StringLiteral(String::from("This is a test")),
                Token::Number(1234),
                Token::Identifier(String::from("Hello_World")),
            ])
        );
    }

    #[test]
    fn tokenization_error() {
        let code = "
            =0b11110000 ; Example of a comment
        ";
        assert_eq!(
            tokenize(String::from(code)),
            Err(TokenizationError::UnexpectedCharacter)
        );

        let code = "
            0b1111; Example of a comment
        ";
        assert_eq!(
            tokenize(String::from(code)),
            Err(TokenizationError::IncompleteByte)
        );
    }
}