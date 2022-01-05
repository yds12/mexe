/// Represents any errors that may occur in this library
pub enum MexeError {
    /// Invalid character at the specified index
    InvalidCharacter(usize),
}

/// Represents the result of any fallible operation in this library
pub type Result<T> = std::result::Result<T, MexeError>;

/// Evaluates a numeric expression.
///
/// The expression can contain integers, floats, sums, subtractions,
/// multiplications, divisions and can use parentheses. Whitespace is ignored.
pub fn eval(expression: &str) -> Result<f64> {
    unimplemented!();
}

/// Evaluates a numeric expression assuming it is just one operation between
/// two numbers, without parentheses. Whitespace is ignored.
pub fn eval_binary(expression: &str) -> Result<f64> {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval() {
        assert_eq!(18.0, eval("1 + (4 * 5) - 9 / 3"));
        assert_eq!(8.4, eval("(1 + (4 * 5)) / 2 - 3 * 0.7"));
        assert_eq!(9.9, eval("(1 + ((4 * 5) + (3))) / 2 - 3 * 0.7"));
    }

    #[test]
    fn test_eval_binary() {
        assert_eq!(3.0, eval_binary("1 + 2"));
        assert_eq!(10.0, eval_binary("2*5"));
        assert_eq!(1.1, eval_binary("5.5/5"));
        assert_eq!(10.5, eval_binary(" 5.5  + 5 "));
    }
}
