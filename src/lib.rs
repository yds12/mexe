/// Evaluates a numeric expression.
///
/// The expression can contain integers, floats, sums, subtractions,
/// multiplications, divisions and can use parentheses. Whitespace is ignored.
pub fn eval(expression: &str) -> f64 {
    unimplemented!();
}

/// Evaluates a numeric expression assuming it is just one operation between
/// two numbers, without parentheses. Whitespace is ignored.
pub fn eval_binary(expression: &str) -> f64 {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval() {
        let expr = "1 + (4 * 5) - 9 / 3";
        assert_eq!(eval(expr), 18.0);

        let expr = "(1 + (4 * 5)) / 2 - 3 * 0.7";
        assert_eq!(eval(expr), 8.4);
    }

    #[test]
    fn test_eval_binary() {
        let expr = "1 + 2";
        assert_eq!(eval(expr), 3.0);

        let expr = "2*5";
        assert_eq!(eval(expr), 10.0);

        let expr = "5.5/5";
        assert_eq!(eval(expr), 1.1);
    }
}
