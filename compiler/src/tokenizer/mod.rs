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
    Comment(String),
}

type CharIterator<'a> = std::iter::Peekable<std::str::Chars<'a>>;

mod binary_byte;
mod string_literal;
mod comment;

pub fn tokenize(contents: String) -> Result<Vec<Token>, TokenizationError> {
    let mut tokens = Vec::<Token>::new();
    let mut char_iter = contents.chars().peekable();

    while let Some(c) = char_iter.peek() {
        tokens.push(match c {
            '1' | '0' => binary_byte::parse(&mut char_iter)?,
            ';' => comment::parse(&mut char_iter)?,
            '"' => string_literal::parse(&mut char_iter)?,
            c if c.is_whitespace() => {
                char_iter.next();
                continue;
            },
            _ => return Err(TokenizationError::UnexpectedCharacter)
        });
    }

    return Ok(tokens);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizes_codeblock() {
        let code = r#"
            11110000 ; Example of a comment
            00000001 10001000
            ; On its own
            11111111 ; Can have 1s and 0s in comment
            "This is a test"
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