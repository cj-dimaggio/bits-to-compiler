use super::*;
use std::ops::Range;
use std::collections::VecDeque;

fn is_operator(token: &Token) -> bool {
    match token {
        Token::Plus | Token::Minus | Token::Multiply => true,
        _ => false
    }
}

fn precedence(token: &Token) -> u8 {
    match token {
        Token::Plus | Token::Minus => 0,
        Token::Multiply => 1,
        _ => panic!("Token {:?} is not an operator", token)
    }
}

fn to_reverse_polish_notation(tokens: &Vec<Token>, range: Range<usize>) -> Result<VecDeque<Token>, TokenizationError> {
    let mut output_queue = VecDeque::new();
    let mut operator_stack = Vec::<Token>::new();

    // This is a little funky.
    for i in range {
        let token = tokens[i].clone();
        match token {
            Token::Number(_) => output_queue.push_back(token),
            _ if is_operator(&token) => {
                while let Some(top) = operator_stack.last() {
                    // All of our operations happen to be left-associative so we don't need to check for it
                    if is_operator(&top) && ((precedence(top) > precedence(&token)) || (precedence(top) == precedence(&token))) {
                        output_queue.push_back(operator_stack.pop().unwrap());
                    } else {
                        break;
                    }
                }

                operator_stack.push(token)
            },
            Token::OpenParen => operator_stack.push(token),
            Token::CloseParen => {
                loop {
                    let operator = operator_stack.pop();
                    match operator {
                        Some(Token::OpenParen) => break,
                        Some(operator) => output_queue.push_back(operator),
                        None => return Err(TokenizationError::MismatchedParen)
                    }
                }
            },
            _ => panic!("to_reverse_polish_notation called with invalid token {:?}", token),
        }
    }

    while let Some(operator) = operator_stack.pop() {
        match operator {
            Token::OpenParen | Token::CloseParen => return Err(TokenizationError::MismatchedParen),
            _ => output_queue.push_back(operator)
        }
    }

    Ok(output_queue)
}

fn calculate(inputs: &mut VecDeque<Token>) -> Result<i32, TokenizationError> {
    let mut stack = Vec::<i32>::new();

    while let Some(token) = inputs.pop_front() {
        match token {
            Token::Number(n) => stack.push(n),
            _ if is_operator(&token) => {
                let right = stack.pop().expect("Arithmetic malformed");
                let left = stack.pop().expect("Arithmetic malformed");
        
                match token {
                    Token::Plus => stack.push(right + left),
                    Token::Minus => stack.push(left - right),
                    Token::Multiply => stack.push(right * left),
                    _ => continue
                }
            },
            _ => panic!("calculate called with invalid token {:?}", token),
        }
    }

    if stack.len() != 1 {
        return Err(TokenizationError::InvalidArithmetic)
    }

    Ok(stack.pop().unwrap())
}

fn find_equation(tokens: &Vec<Token>) -> Result<Option<Range<usize>>, TokenizationError> {
    let mut start = None;
    for (i, token) in tokens.iter().enumerate() {
        if let Token::OpenParen = token {
            start = Some(i);
            break
        }
    }
    
    if let Some(start) = start {
        let mut parens = 0;
        let mut end = None;
        
        for i in start..tokens.len() {
            // We should really be enforcing a stricter syntax
            match tokens[i]{
                Token::OpenParen => parens += 1,
                Token::CloseParen => {
                    parens -= 1;
                    if parens == 0 {
                        end = Some(i + 1);
                        break
                    }
                },
                Token::Number(_) | Token::Plus | Token::Minus | Token::Multiply => continue,
                _ => return Err(TokenizationError::UnexpectedCharacter)
            }
        }

        if let Some(end) = end {
            Ok(Some(start..end))
        } else {
            Err(TokenizationError::MismatchedParen)
        }
    } else {
        Ok(None)
    }
}

pub fn perform_calculations(mut tokens: Vec<Token>) -> Result<Vec<Token>, TokenizationError> {
    if let Some(range) = find_equation(&tokens)? {
        let mut polish = to_reverse_polish_notation(&tokens, range.clone())?;
        tokens.drain(range.clone());
        let result = Token::Number(calculate(&mut polish)?);
        tokens.insert(range.start, result);
        Ok(tokens)
    } else {
        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_simple_equation() {
        assert_eq!(
            perform_calculations(vec![
                Token::Times,
                Token::OpenParen,
                Token::Number(5),
                Token::Plus,
                Token::Number(4),
                Token::Multiply,
                Token::Number(10),
                Token::Minus,
                Token::Number(10),
                Token::CloseParen,
                Token::Binary(0x44)
            ]),
            Ok(vec![
                Token::Times,
                Token::Number(35),
                Token::Binary(0x44),
            ])
        );
    }

    #[test]
    fn calculates_with_parens() {
        assert_eq!(
            perform_calculations(vec![
                Token::Times,
                Token::OpenParen,
                Token::OpenParen,
                Token::Number(5),
                Token::Plus,
                Token::Number(4),
                Token::CloseParen,
                Token::Multiply,
                Token::Number(10),
                Token::Minus,
                Token::Number(10),
                Token::CloseParen,
                Token::Binary(0x44)
            ]),
            Ok(vec![
                Token::Times,
                Token::Number(80),
                Token::Binary(0x44),
            ])
        );
    }
}
