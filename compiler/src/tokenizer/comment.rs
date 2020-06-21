use super::*;

// Shouldn't do anything really but move the iterator forward past the comment
pub fn parse(char_iter: &mut CharIterator) -> Result<(), TokenizationError> {
    let mut comment = String::new();

    // Skip the starting ';' char
    assert_eq!(char_iter.next(), Some(';'));

    while let Some(c) = char_iter.next() {
        // We don't want to swallow newlines
        if c == '\n' {
            break;
        }
        
        comment.push(c)
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_comment() {
        let code = ";hello world";
        assert_eq!(
            parse(&mut code.chars().peekable()),
            Ok(())
        );
    }

    #[test]
    fn extracts_comment_with_newline() {
        let code = ";hello world\ntest";
        assert_eq!(
            parse(&mut code.chars().peekable()),
            Ok(())
        );
    }
}
