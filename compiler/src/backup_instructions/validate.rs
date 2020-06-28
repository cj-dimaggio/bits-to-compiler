macro_rules! validate_syntax {
    ( $expression:expr , $($pattern:pat)|+ ) => {
        match $expression {
            $($pattern)|+ => Ok(()),
            _ => Err(SyntaxError::InvalidParam)
        }
    };

    ( $expression:expr , $pat:pat => $arm:expr ) => {
        match $expression {
            $pat => Ok($arm),
            _ => Err(SyntaxError::InvalidParam)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::super::SyntaxError;

    #[allow(dead_code)]
    enum ForTesting {
        Foo,
        Bar,
        None,
    }

    #[test]
    fn macro_passes_if_there_is_a_match() {
        let x = vec![5];
        assert_eq!(
            validate_syntax!(x.get(0), Some(5)),
            Ok(())
        );
    }

    #[test]
    fn macro_passes_with_multiple_match() {
        let x = ForTesting::Bar;
        assert_eq!(
            validate_syntax!(x, ForTesting::Foo | ForTesting::Bar),
            Ok(())
        );
    }

    #[test]
    fn macro_fails_if_there_is_not_a_match() {
        let x = vec![];
        assert_eq!(
            validate_syntax!(x.get(0), Some(5)),
            Err(SyntaxError::InvalidParam)
        )
    }

    #[test]
    fn macro_runs_and_returns_expr() {
        let x = vec![5];
        assert_eq!(
            validate_syntax!(x.get(0), Some(y) => *y),
            Ok(5)
        )
    }
}
