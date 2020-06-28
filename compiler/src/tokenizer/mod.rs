#[derive(Debug, PartialEq, Eq)]
pub enum TokenizationError {
    UnexpectedCharacter,
    UnterminatedStringLiteral,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Semicolon,
    Number(i16),
    QuotedString(String),
    Identifier(String),
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    Equals,
    While,
    Let,
    Function,
    Plus,
    DoesNotEqual,
}

type CharIterator<'a> = std::iter::Peekable<std::str::Chars<'a>>;

fn one_char_token(token: Token, char_iter: &mut CharIterator) -> Token {
    char_iter.next();
    token
}

mod string_literal;
mod alphanumeric;

pub fn tokenize(contents: String) -> Result<Vec<Token>, TokenizationError> {
    let mut tokens = Vec::<Token>::new();
    let mut char_iter = contents.chars().peekable();

    while let Some(&c) = char_iter.peek() {
        tokens.push(match c {
            '(' => one_char_token(Token::OpenParen, &mut char_iter),
            ')' => one_char_token(Token::CloseParen, &mut char_iter),
            '{' => one_char_token(Token::OpenBrace, &mut char_iter),
            '}' => one_char_token(Token::CloseBrace, &mut char_iter),
            '[' => one_char_token(Token::OpenBracket, &mut char_iter),
            ']' => one_char_token(Token::CloseBracket, &mut char_iter),
            ';' => one_char_token(Token::Semicolon, &mut char_iter),
            '=' => one_char_token(Token::Equals, &mut char_iter),
            '+' => one_char_token(Token::Plus, &mut char_iter),
            '!' => {
                char_iter.next();
                match char_iter.next() {
                    Some('=') => Token::DoesNotEqual,
                    _ => return Err(TokenizationError::UnexpectedCharacter)
                }
            },
            '"' => string_literal::parse(&mut char_iter)?,
            c if alphanumeric::is_alphanumeric(c) => alphanumeric::parse(&mut char_iter)?,
            c if c.is_whitespace() => {
                char_iter.next();
                continue;
            },
            _ => return Err(TokenizationError::UnexpectedCharacter)
        });
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizes_codeblock() {
        let code = r#"
            let hello_world = "Hello, World!";

            fn main() {
                let i = 0;
                while (hello_world[i] != 0) {
                    print(hello_world[i]);
                    i = i + 1;
                }
            }
        "#;

        assert_eq!(
            tokenize(String::from(code)),
            Ok(vec![
                Token::Let, Token::Identifier("hello_world".to_string()), Token::Equals, Token::QuotedString("Hello, World!".to_string()), Token::Semicolon,
                Token::Function, Token::Identifier("main".to_string()), Token::OpenParen, Token::CloseParen, Token::OpenBrace,
                Token::Let, Token::Identifier("i".to_string()), Token::Equals, Token::Number(0), Token::Semicolon,
                Token::While, Token::OpenParen, Token::Identifier("hello_world".to_string()), Token::OpenBracket, Token::Identifier("i".to_string()), Token::CloseBracket, Token::DoesNotEqual, Token::Number(0), Token::CloseParen, Token::OpenBrace,
                Token::Identifier("print".to_string()), Token::OpenParen, Token::Identifier("hello_world".to_string()), Token::OpenBracket, Token::Identifier("i".to_string()), Token::CloseBracket, Token::CloseParen, Token::Semicolon,
                Token::Identifier("i".to_string()), Token::Equals, Token::Identifier("i".to_string()), Token::Plus, Token::Number(1), Token::Semicolon,
                Token::CloseBrace,
                Token::CloseBrace,
            ])
        );
    }
}
