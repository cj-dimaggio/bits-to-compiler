#[derive(Debug, PartialEq, Eq)]
pub enum TokenizationError {
    UnexpectedCharacter,
    MalformedByte,
    UnterminatedStringLiteral,
    InvalidHex,
    MismatchedParen,
    InvalidArithmetic
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Binary(u8),
    QuotedString(String),
    Number(i32),
    Label(String),
    Reference(String),
    Times,
    Offset,
    Cli,
    Hlt,
    Lodsb,
    Interrupt,
    Org,
    Register8(String),
    Register16(String),
    Mov,
    Or,
    Jz(u16), // Keep track of token's binary position
    Jmp(u16), // Keep track of token's binary position
    OpenParen,
    CloseParen,
    Plus,
    Minus,
    Multiply,
}

type CharIterator<'a> = std::iter::Peekable<std::str::Chars<'a>>;

mod binary_byte;
mod string_literal;
mod alphanumeric;
mod comment;
mod location;
mod arithmetic;

fn one_char_token(token: Token, char_iter: &mut CharIterator,) -> Token {
    char_iter.next();
    token
}

pub fn tokenize(contents: String, current_location: u16, starting_location: u16) -> Result<Vec<Token>, TokenizationError> {
    let mut tokens = Vec::<Token>::new();
    let mut char_iter = contents.chars().peekable();

    while let Some(&c) = char_iter.peek() {
        tokens.push(match c {
            ';' => { 
                comment::parse(&mut char_iter)?;
                continue;
            },
            '"' => string_literal::parse(&mut char_iter)?,
            '$' => location::parse(&mut char_iter, current_location, starting_location)?,
            '(' => one_char_token(Token::OpenParen, &mut char_iter),
            ')' => one_char_token(Token::CloseParen, &mut char_iter),
            '+' => one_char_token(Token::Plus, &mut char_iter),
            '-' => one_char_token(Token::Minus, &mut char_iter),
            '*' => one_char_token(Token::Multiply, &mut char_iter),
            c if alphanumeric::is_alphanumeric(c) => alphanumeric::parse(&mut char_iter, current_location)?,
            c if c.is_whitespace() => {
                char_iter.next();
                continue;
            },
            _ => return Err(TokenizationError::UnexpectedCharacter)
        });
    }

    arithmetic::perform_calculations(tokens)
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
            Hello_World:
            TIMES 5 0b10001000
            Hello_World
        "#;
        assert_eq!(
            tokenize(String::from(code), 0, 0),
            Ok(vec![
                Token::Binary(0b11110000),
                Token::Binary(0b00000001),
                Token::Binary(0b10001000),
                Token::Binary(0b11111111),
                Token::QuotedString(String::from("This is a test")),
                Token::Number(1234),
                Token::Label(String::from("Hello_World")),
                Token::Times,
                Token::Number(5),
                Token::Binary(0b10001000),
                Token::Reference(String::from("Hello_World")),
            ])
        );
    }

    #[test]
    fn tokenization_error() {
        let code = "
            =0b11110000 ; Example of a comment
        ";
        assert_eq!(
            tokenize(String::from(code), 0, 0),
            Err(TokenizationError::UnexpectedCharacter)
        );

        let code = "
            0b1111; Example of a comment
        ";
        assert_eq!(
            tokenize(String::from(code), 0, 0),
            Err(TokenizationError::MalformedByte)
        );
    }

    #[test]
    fn reduces_arithmetic() {
        let code = "TIMES ( 5 * 2 + 8 ) 0b11111111";
        assert_eq!(
            tokenize(String::from(code), 0, 0),
            Ok(vec![
                Token::Times,
                Token::Number(18),
                Token::Binary(0b11111111)
            ])
        );
    }
}