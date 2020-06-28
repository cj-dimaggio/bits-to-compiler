use super::tokenizer::Token;

#[derive(Debug, PartialEq, Eq)]
pub enum SyntaxError {
    UnexpectedToken
}

type TokenIterator<'a> = std::iter::Peekable<std::slice::Iter<'a, Token>>;

#[macro_use]
mod validate;

mod statement;
mod function;
mod expression;

#[derive(Debug, PartialEq, Eq)]
pub struct Program {
    functions: Vec::<function::Function>,
    statements: Vec::<statement::Statement>,
}

pub fn parse(tokens: Vec::<Token>) -> Result<Program, SyntaxError> {
    let mut token_iter = tokens.iter().peekable();

    let mut functions = vec![];
    let mut statements = vec![];

    while let Some(token) = token_iter.peek() {
        if let Token::Function = token {
            let function = function::parse(&mut token_iter)?;
            functions.push(function);
        }
        else if let Some(statement) = statement::parse(&mut token_iter)? {
            statements.push(statement)
        } else {
            return Err(SyntaxError::UnexpectedToken)
        }
    }

    Ok(Program {
        functions,
        statements,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::tokenizer;

    #[test]
    fn parse_codeblock() {
        let code = r#"
                while (hello_world[i] != 0) {}
        "#.to_string();

        assert_eq!(
            parse(tokenizer::tokenize(code).unwrap()),
            Ok(Program {
                statements: vec![],
                functions: vec![],
            })
        );
    }
}

