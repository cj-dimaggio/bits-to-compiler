#[derive(Debug, PartialEq, Eq)]
pub enum TokenizationError {
    UnexpectedCharacter,
    IncompleteByte,
    UnterminatedStringLiteral
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Binary([bool; 8]),
    QuotedString(String),
    Number(i64),
    Identifier(String),
    Multiply,
    Newline,
}

type CharIterator<'a> = itertools::MultiPeek<std::str::Chars<'a>>;

mod binary_byte;
mod string_literal;
mod alphanumeric;
mod comment;

fn simple_symbol(token: Token, char_iter: &mut CharIterator) -> Token {
    char_iter.next();
    token
}

pub fn tokenize(contents: String) -> Result<Vec<Token>, TokenizationError> {
    let mut tokens = Vec::<Token>::new();
    let mut char_iter = itertools::multipeek(contents.chars());

    while let Some(&c) = char_iter.peek() {
        // Make sure our parsers can grab the character we just pulled
        char_iter.reset_peek();

        tokens.push(match c {
            ';' => comment::parse(&mut char_iter)?,
            '"' => string_literal::parse(&mut char_iter)?,
            '*' => simple_symbol(Token::Multiply, &mut char_iter),
            '\n' => simple_symbol(Token::Newline, &mut char_iter),
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
            0b10001000 * 5
        "#;
        assert_eq!(
            tokenize(String::from(code)),
            Ok(vec![
                Token::Newline,
                Token::Binary([true, true, true, true, false, false, false, false]),
                Token::Newline,
                Token::Binary([false, false, false, false, false, false, false, true]),
                Token::Binary([true, false, false, false, true, false, false, false]),
                Token::Newline,
                Token::Newline,
                Token::Binary([true, true, true, true, true, true, true, true]),
                Token::Newline,
                Token::QuotedString(String::from("This is a test")),
                Token::Newline,
                Token::Number(1234),
                Token::Newline,
                Token::Identifier(String::from("Hello_World")),
                Token::Newline,
                Token::Binary([true, false, false, false, true, false, false, false]),
                Token::Multiply,
                Token::Number(5),
                Token::Newline,
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